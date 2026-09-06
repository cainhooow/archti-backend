use crate::{
    application::exceptions::{AppError, AppResult},
    domain::{
        entities::{permission::Permission, permission_catalog::catalog::DEFAULT_PERMISSIONS},
        exceptions::RepositoryError,
        repositories::permission_repository_trait::{
            PermissionCreateRepository, PermissionReadRepository,
        },
    },
};

pub struct BootstrapPermissionsResult {
    pub created: Vec<Permission>,
    pub skipped_codes: Vec<String>,
}

pub struct BootstrapPermissionsUseCase<R>
where
    R: PermissionReadRepository + PermissionCreateRepository,
{
    repository: R,
}

impl<R> BootstrapPermissionsUseCase<R>
where
    R: PermissionReadRepository + PermissionCreateRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> AppResult<BootstrapPermissionsResult> {
        let mut created = Vec::new();
        let mut skipped_codes = Vec::new();

        for definition in DEFAULT_PERMISSIONS {
            let code = definition.code();

            match self.repository.by_code(&code).await {
                Ok(_) => {
                    skipped_codes.push(code);
                    continue;
                }
                Err(RepositoryError::NotFound) => {}
                Err(err) => return Err(err.into()),
            }

            let permission = Permission::create(
                code,
                definition.module().to_string(),
                definition.action().to_string(),
                Some(definition.description().to_string()),
            )
            .map_err(AppError::RuleViolation)?;

            created.push(self.repository.create(&permission).await?);
        }

        Ok(BootstrapPermissionsResult {
            created,
            skipped_codes,
        })
    }
}
