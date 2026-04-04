use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
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

use crate::infrastructure::entities::{
    company_membership, membership_role, permission, role, role_permission,
};

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
        let company_id = Uuid::from_str(membership.company_id())
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;
        let user_id = Uuid::from_str(membership.user_id())
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        let model = company_membership::ActiveModel {
            id: Set(Uuid::new_v4()),
            company_id: Set(company_id),
            user_id: Set(user_id),
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
        let membership_id =
            Uuid::from_str(membership_id).map_err(|err| RepositoryError::Generic(err.to_string()))?;

        match company_membership::Entity::find_by_id(membership_id)
            .one(&*self.conn)
            .await
        {
            Ok(Some(data)) => Ok(data.try_into().map_err(RepositoryError::Generic)?),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn by_company_and_user(
        &self,
        company_id: &str,
        user_id: &str,
    ) -> Result<CompanyMembership, RepositoryError> {
        let company_id =
            Uuid::from_str(company_id).map_err(|err| RepositoryError::Generic(err.to_string()))?;
        let user_id =
            Uuid::from_str(user_id).map_err(|err| RepositoryError::Generic(err.to_string()))?;

        match company_membership::Entity::find()
            .filter(company_membership::Column::CompanyId.eq(company_id))
            .filter(company_membership::Column::UserId.eq(user_id))
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
    async fn assign_role(&self, membership_id: &str, role_id: &str) -> Result<(), RepositoryError> {
        let membership = self.by_id(membership_id).await?;
        let membership_uuid =
            Uuid::from_str(membership.id().ok_or(RepositoryError::NotFound)?)
                .map_err(|err| RepositoryError::Generic(err.to_string()))?;
        let role_uuid =
            Uuid::from_str(role_id).map_err(|err| RepositoryError::Generic(err.to_string()))?;

        let role_model = match role::Entity::find_by_id(role_uuid).one(&*self.conn).await {
            Ok(Some(model)) => model,
            Ok(None) => return Err(RepositoryError::NotFound),
            Err(err) => return Err(RepositoryError::Generic(err.to_string())),
        };

        // Never allow cross-company role assignment, even if a valid role id is provided.
        if role_model.company_id.to_string() != membership.company_id() {
            return Err(RepositoryError::Generic(
                "Role does not belong to the same company as the membership".to_string(),
            ));
        }

        let existing = membership_role::Entity::find_by_id((membership_uuid, role_uuid))
            .one(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        if existing.is_some() {
            return Ok(());
        }

        let model = membership_role::ActiveModel {
            membership_id: Set(membership_uuid),
            role_id: Set(role_uuid),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(_) => Ok(()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn has_permission(
        &self,
        company_id: &str,
        user_id: &str,
        permission_code: &str,
    ) -> Result<bool, RepositoryError> {
        let membership = match self.by_company_and_user(company_id, user_id).await {
            Ok(membership) => membership,
            Err(RepositoryError::NotFound) => return Ok(false),
            Err(err) => return Err(err),
        };

        if !membership.is_active() {
            return Ok(false);
        }

        let membership_id =
            Uuid::from_str(membership.id().ok_or(RepositoryError::NotFound)?)
                .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        let membership_roles = membership_role::Entity::find()
            .filter(membership_role::Column::MembershipId.eq(membership_id))
            .all(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        if membership_roles.is_empty() {
            return Ok(false);
        }

        let role_ids: Vec<Uuid> = membership_roles.into_iter().map(|model| model.role_id).collect();

        let scoped_roles = role::Entity::find()
            .filter(role::Column::Id.is_in(role_ids.clone()))
            .filter(
                role::Column::CompanyId.eq(
                    Uuid::from_str(company_id)
                        .map_err(|err| RepositoryError::Generic(err.to_string()))?,
                ),
            )
            .all(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        if scoped_roles.is_empty() {
            return Ok(false);
        }

        let scoped_role_ids: Vec<Uuid> = scoped_roles.into_iter().map(|model| model.id).collect();

        let role_permissions = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(scoped_role_ids))
            .all(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        if role_permissions.is_empty() {
            return Ok(false);
        }

        let permission_ids: Vec<Uuid> = role_permissions
            .into_iter()
            .map(|model| model.permission_id)
            .collect();

        let permission = permission::Entity::find()
            .filter(permission::Column::Id.is_in(permission_ids))
            .filter(permission::Column::Code.eq(permission_code))
            .one(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        Ok(permission.is_some())
    }
}
