use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repositories::membership_repository_trait::{
    MembershipReadRepository, MembershipRoleRepository,
};
use crate::domain::{
    entities::company_membership::CompanyMembership, exceptions::RepositoryError,
    repositories::membership_repository_trait::CreateMembershipRepository,
};

use crate::infrastructure::entities::company_membership;
use crate::infrastructure::entities::membership_role;

pub struct SeaOrmMembershipRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmMembershipRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CreateMembershipRepository for SeaOrmMembershipRepository {
    async fn create_membership(
        &self,
        membership: &CompanyMembership,
    ) -> Result<CompanyMembership, RepositoryError> {
        let model = company_membership::ActiveModel {
            id: Set(Uuid::new_v4()),
            company_id: Set(Uuid::from_str(membership.company_id()).unwrap()),
            user_id: Set(Uuid::from_str(membership.user_id()).unwrap()),
            membership_type: Set(membership.membership_type().as_str().to_string()),
            status_key: Set(membership.status().as_str().to_string()),
            display_name: Set(membership.display_name()),
            invited_at: Set(membership.invited_at()),
            accepted_at: Set(membership.accepted_at()),
            last_seen_at: Set(membership.last_seen_at()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(data.try_into().map_err(RepositoryError::Generic)?),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl MembershipReadRepository for SeaOrmMembershipRepository {
    async fn by_id(&self, membership_id: &str) -> Result<CompanyMembership, RepositoryError> {
        match company_membership::Entity::find_by_user_id(Uuid::from_str(membership_id).unwrap())
            .one(&*self.conn)
            .await
        {
            Ok(Some(data)) => Ok(data.try_into().map_err(RepositoryError::Generic)?),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl MembershipRoleRepository for SeaOrmMembershipRepository {
    async fn assign_role(
        &self,
        membership_id: String,
        role_id: String,
    ) -> Result<(), RepositoryError> {
        let membership = self.by_id(membership_id.as_str()).await?;

        let model = membership_role::ActiveModel {
            membership_id: Set(Uuid::from_str(membership.id().unwrap()).unwrap()),
            role_id: Set(Uuid::from_str(role_id.as_str()).unwrap()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(_) => Ok(()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
