use std::sync::Arc;

use crate::domain::{entities::company_membership::CompanyMembership, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait CreateMembershipRepository: Send + Sync {
    async fn create_membership(
        &self,
        membership: &CompanyMembership,
    ) -> Result<CompanyMembership, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> CreateMembershipRepository for Arc<T>
where
    T: CreateMembershipRepository + ?Sized,
{
    async fn create_membership(
        &self,
        membership: &CompanyMembership,
    ) -> Result<CompanyMembership, RepositoryError> {
        (**self).create_membership(membership).await
    }
}

#[async_trait::async_trait]
pub trait MembershipUpdateRepository: Send + Sync {
    async fn update_membership(
        &self,
        membership: &CompanyMembership,
    ) -> Result<CompanyMembership, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> MembershipUpdateRepository for Arc<T>
where
    T: MembershipUpdateRepository + ?Sized,
{
    async fn update_membership(
        &self,
        membership: &CompanyMembership,
    ) -> Result<CompanyMembership, RepositoryError> {
        (**self).update_membership(membership).await
    }
}

#[async_trait::async_trait]
pub trait MembershipReadRepository: Send + Sync {
    async fn by_id(&self, membership_id: &str) -> Result<CompanyMembership, RepositoryError>;
    async fn by_company_and_user(
        &self,
        company_id: &str,
        user_id: &str,
    ) -> Result<CompanyMembership, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> MembershipReadRepository for Arc<T>
where
    T: MembershipReadRepository + ?Sized,
{
    async fn by_id(&self, membership_id: &str) -> Result<CompanyMembership, RepositoryError> {
        (**self).by_id(membership_id).await
    }

    async fn by_company_and_user(
        &self,
        company_id: &str,
        user_id: &str,
    ) -> Result<CompanyMembership, RepositoryError> {
        (**self).by_company_and_user(company_id, user_id).await
    }
}

#[async_trait::async_trait]
pub trait MembershipRoleRepository: Send + Sync {
    async fn assign_role(&self, membership_id: &str, role_id: &str) -> Result<(), RepositoryError>;
    async fn has_permission(
        &self,
        company_id: &str,
        user_id: &str,
        permission_code: &str,
    ) -> Result<bool, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> MembershipRoleRepository for Arc<T>
where
    T: MembershipRoleRepository + ?Sized,
{
    async fn assign_role(&self, membership_id: &str, role_id: &str) -> Result<(), RepositoryError> {
        (**self).assign_role(membership_id, role_id).await
    }

    async fn has_permission(
        &self,
        company_id: &str,
        user_id: &str,
        permission_code: &str,
    ) -> Result<bool, RepositoryError> {
        (**self)
            .has_permission(company_id, user_id, permission_code)
            .await
    }
}
