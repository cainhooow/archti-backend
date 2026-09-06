use std::sync::Arc;

use archti_backend::{
    application::usecases::system::create_permission_usecase::{
        CreatePermissionCommand as CreatePermissionInput, CreatePermissionUseCase,
    },
    infrastructure::persistence::sea_orm_permission_repository::SeaOrmPermissionRepository,
};
use sea_orm::DatabaseConnection;

use crate::cli::prompt::{confirm, prompt, prompt_optional};

pub struct CreatePermissionCliCommand {
    pub db: Arc<DatabaseConnection>,
}

impl CreatePermissionCliCommand {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, params: &[String]) -> Result<(), String> {
        let (module, action, description) = if params.len() >= 2 {
            let description = if params.len() > 2 {
                Some(params[2..].join(" "))
            } else {
                None
            };

            (params[0].clone(), params[1].clone(), description)
        } else {
            (
                prompt("Module")?,
                prompt("Action")?,
                prompt_optional("Description (optional)")?,
            )
        };

        let code = format!("{}.{}", module.trim(), action.trim());

        println!("code: {code}");
        println!("module: {}", module.trim());
        println!("action: {}", action.trim());
        println!("description: {}", description.as_deref().unwrap_or("-"));

        if !confirm("Proceed? [y/N]")? {
            return Err("Operation cancelled".into());
        }

        let repository = SeaOrmPermissionRepository::new(self.db.clone());
        let usecase = CreatePermissionUseCase::new(repository);

        let permission = usecase
            .execute(CreatePermissionInput {
                code,
                module,
                action,
                description,
            })
            .await
            .map_err(|err| err.to_string())?;

        println!("Permission created: {}", permission.code());
        Ok(())
    }
}
