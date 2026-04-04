use crate::{
    application::exceptions::{AppError, AppResult},
    domain::{
        entities::permission::Permission,
        repositories::permission_repository_trait::{
            PermissionCreateRepository, PermissionReadRepository,
        },
    },
};

pub struct CreatePermissionCommand {
    pub code: String,
    pub module: String,
    pub action: String,
    pub description: Option<String>,
}

pub struct CreatePermissionUseCase<R>
where
    R: PermissionReadRepository + PermissionCreateRepository,
{
    repository: R,
}

impl<R> CreatePermissionUseCase<R>
where
    R: PermissionReadRepository + PermissionCreateRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreatePermissionCommand) -> AppResult<Permission> {
        if self.repository.by_code(&command.code).await.is_ok() {
            return Err(AppError::Conflict(format!(
                "Permission with code '{}' already exists",
                command.code
            )));
        }

        let permission = Permission::create(
            command.code,
            command.module,
            command.action,
            command.description,
        )?;

        let permission = self.repository.create(&permission).await?;
        Ok(permission)
    }
}
