use crate::{
    application::exceptions::{AppError, AppResult},
    domain::{
        entities::permission::Permission,
        exceptions::RepositoryError,
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
        match self.repository.by_code(&command.code).await {
            Ok(_) => {
                return Err(AppError::Conflict(format!(
                    "Permission with code '{}' already exists",
                    command.code
                )));
            }
            Err(RepositoryError::NotFound) => {}
            Err(err) => return Err(err.into()),
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
