use std::sync::Arc;

use crate::domain::{entities::permission::Permission, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait PermissionCreateRepository: Send + Sync {
    async fn create(&self, permission: &Permission) -> Result<Permission, RepositoryError>;
}

#[async_trait::async_trait]
pub trait PermissionReadRepository: Send + Sync {
    async fn by_code(&self, code: &str) -> Result<Permission, RepositoryError>;
    async fn by_id(&self, id: &i64) -> Result<Permission, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> PermissionCreateRepository for Arc<T>
where
    T: PermissionCreateRepository + ?Sized,
{
    async fn create(&self, permission: &Permission) -> Result<Permission, RepositoryError> {
        (**self).create(permission).await
    }
}

#[async_trait::async_trait]
impl<T> PermissionReadRepository for Arc<T>
where
    T: PermissionReadRepository + ?Sized,
{
    async fn by_code(&self, code: &str) -> Result<Permission, RepositoryError> {
        (**self).by_code(code).await
    }

    async fn by_id(&self, id: &i64) -> Result<Permission, RepositoryError> {
        (**self).by_id(id).await
    }
}
