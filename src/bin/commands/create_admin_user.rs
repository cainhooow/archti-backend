use std::sync::Arc;

use archti_backend::{
    application::usecases::user::cli::bootstrap_admin_usecase::{
        BootstrapAdminCommand, BootstrapAdminUseCase,
    },
    infrastructure::{
        persistence::sea_orm_user_repository::SeaOrmUserRepository, security::Argon2HasherImpl,
    },
};
use sea_orm::DatabaseConnection;

use crate::cli::prompt::{confirm, prompt, prompt_optional};

pub struct CreateAdminUserCommand {
    pub db: Arc<DatabaseConnection>,
    pub args: Vec<String>,
}

impl CreateAdminUserCommand {
    pub fn new(db: Arc<DatabaseConnection>, args: Vec<String>) -> Self {
        Self { db, args }
    }

    pub async fn handle(&self) -> Result<(), String> {
        let email = prompt("Email")?;
        let full_name = prompt("Full name")?;
        let phone = prompt_optional("Phone (optional)")?;
        let password = prompt("Password")?;

        println!("email: {email}");
        println!("full_name: {full_name}");
        println!("phone: {}", phone.as_deref().unwrap_or("-"));

        if !confirm("Proceed? [y/N]")? {
            return Err("Operation cancelled".into());
        }

        let repository = SeaOrmUserRepository::new(self.db.clone());
        let hasher = Argon2HasherImpl::default();

        let usecase = BootstrapAdminUseCase::new(repository, hasher);
        usecase
            .execute(BootstrapAdminCommand {
                email,
                full_name,
                phone,
                password,
            })
            .await
            .map_err(|err| err.to_string())?;

        Ok(())
    }
}
