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
