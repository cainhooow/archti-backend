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
pub trait MembershipUpdateRepository: Send + Sync {
    async fn update_membership(
        &self,
        membership: &CompanyMembership,
    ) -> Result<CompanyMembership, RepositoryError>;
}

#[async_trait::async_trait]
pub trait MembershipReadRepository: Send + Sync {
    async fn by_id(&self, membership_id: String) -> Result<CompanyMembership, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> MembershipReadRepository for Arc<T>
where
    T: MembershipReadRepository + ?Sized,
{
    async fn by_id(&self, membership_id: String) -> Result<CompanyMembership, RepositoryError> {
        (**self).by_id(membership_id).await
    }
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
