use std::sync::Arc;

use crate::domain::{entities::role::Role, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait RoleCreateRepository: Send + Sync {
    async fn create(&self, role: &Role) -> Result<Role, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> RoleCreateRepository for Arc<T>
where
    T: RoleCreateRepository + ?Sized,
{
    async fn create(&self, role: &Role) -> Result<Role, RepositoryError> {
        (**self).create(role).await
    }
}

#[async_trait::async_trait]
pub trait RoleReadRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Role, RepositoryError>;
    async fn find_by_code(&self, code: &str) -> Result<Role, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> RoleReadRepository for Arc<T>
where
    T: RoleReadRepository + ?Sized,
{
    async fn find_by_id(&self, id: &str) -> Result<Role, RepositoryError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_code(&self, code: &str) -> Result<Role, RepositoryError> {
        (**self).find_by_code(code).await
    }
}
