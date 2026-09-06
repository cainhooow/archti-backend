use std::sync::Arc;

use archti_backend::{
    application::usecases::system::bootstrap_permissions_usecase::BootstrapPermissionsUseCase,
    infrastructure::persistence::sea_orm_permission_repository::SeaOrmPermissionRepository,
};
use sea_orm::DatabaseConnection;

pub struct InitDefaultPermissionsCommand {
    pub db: Arc<DatabaseConnection>,
}

impl InitDefaultPermissionsCommand {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn handle(&self) -> Result<(), String> {
        let repository = SeaOrmPermissionRepository::new(self.db.clone());
        let result = BootstrapPermissionsUseCase::new(repository)
            .execute()
            .await
            .map_err(|err| err.to_string())?;

        println!("created: {}", result.created.len());
        for permission in result.created {
            println!("  + {}", permission.code());
        }

        println!("skipped: {}", result.skipped_codes.len());
        for code in result.skipped_codes {
            println!("  = {code}");
        }

        Ok(())
    }
}
