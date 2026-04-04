use crate::domain::{entities::permission::Permission, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait PermissionCreateRepository {
    async fn create(&self, permission: &Permission) -> Result<Permission, RepositoryError>;
}

#[async_trait::async_trait]
pub trait PermissionReadRepository {
    async fn by_code(&self, code: &str) -> Result<Permission, RepositoryError>;
    async fn by_id(&self, id: &str) -> Result<Permission, RepositoryError>;
}
