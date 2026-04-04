use crate::domain::{entities::permission::Permission, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait PermissionCreateRepository {
    async fn create(&self, permission: &Permission) -> Result<Permission, RepositoryError>;
}
