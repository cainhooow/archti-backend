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
    async fn by_id(&self, id: &i64) -> Result<Role, RepositoryError>;
    async fn by_company_and_code(
        &self,
        company_id: &i64,
        code: &str,
    ) -> Result<Role, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> RoleReadRepository for Arc<T>
where
    T: RoleReadRepository + ?Sized,
{
    async fn by_id(&self, id: &i64) -> Result<Role, RepositoryError> {
        (**self).by_id(id).await
    }

    async fn by_company_and_code(
        &self,
        company_id: &i64,
        code: &str,
    ) -> Result<Role, RepositoryError> {
        (**self).by_company_and_code(company_id, code).await
    }
}

#[async_trait::async_trait]
pub trait RolePermissionRepository: Send + Sync {
    async fn assign_permission(
        &self,
        role_id: &i64,
        permission_code: &str,
    ) -> Result<(), RepositoryError>;
}

#[async_trait::async_trait]
impl<T> RolePermissionRepository for Arc<T>
where
    T: RolePermissionRepository + ?Sized,
{
    async fn assign_permission(
        &self,
        role_id: &i64,
        permission_code: &str,
    ) -> Result<(), RepositoryError> {
        (**self).assign_permission(role_id, permission_code).await
    }
}
