use crate::domain::{
    entities::role::Role,
    exceptions::RepositoryError,
    repositories::role_repository_trait::{
        RoleCreateRepository, RolePermissionRepository, RoleReadRepository,
    },
};
use crate::infrastructure::entities::{permission, role, role_permission};
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;
use uuid::Uuid;

pub struct SeaOrmRoleRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmRoleRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl RoleCreateRepository for SeaOrmRoleRepository {
    async fn create(&self, role: &Role) -> Result<Role, RepositoryError> {
        let company_id = Uuid::parse_str(role.company_id())
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        let model = role::ActiveModel {
            id: Set(Uuid::new_v4()),
            company_id: Set(company_id),
            code: Set(role.code().to_string()),
            name: Set(role.name().to_string()),
            description: Set(role.description().map(str::to_string)),
            is_system_role: Set(role.is_system_role()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(Role::from(data)),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl RoleReadRepository for SeaOrmRoleRepository {
    async fn by_id(&self, id: &str) -> Result<Role, RepositoryError> {
        let id = Uuid::parse_str(id).map_err(|err| RepositoryError::Generic(err.to_string()))?;

        match role::Entity::find_by_id(id).one(&*self.conn).await {
            Ok(Some(data)) => Ok(Role::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn by_company_and_code(
        &self,
        company_id: &str,
        code: &str,
    ) -> Result<Role, RepositoryError> {
        let company_id =
            Uuid::parse_str(company_id).map_err(|err| RepositoryError::Generic(err.to_string()))?;

        match role::Entity::find()
            .filter(role::Column::CompanyId.eq(company_id))
            .filter(role::Column::Code.eq(code))
            .one(&*self.conn)
            .await
        {
            Ok(Some(data)) => Ok(Role::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl RolePermissionRepository for SeaOrmRoleRepository {
    async fn assign_permission(
        &self,
        role_id: &str,
        permission_code: &str,
    ) -> Result<(), RepositoryError> {
        let role = self.by_id(role_id).await?;
        let role_id = Uuid::parse_str(role.id().ok_or(RepositoryError::NotFound)?)
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        let permission_model = match permission::Entity::find()
            .filter(permission::Column::Code.eq(permission_code))
            .one(&*self.conn)
            .await
        {
            Ok(Some(model)) => model,
            Ok(None) => return Err(RepositoryError::NotFound),
            Err(err) => return Err(RepositoryError::Generic(err.to_string())),
        };

        let existing = role_permission::Entity::find_by_id((role_id, permission_model.id))
            .one(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        if existing.is_some() {
            return Ok(());
        }

        let model = role_permission::ActiveModel {
            role_id: Set(role_id),
            permission_id: Set(permission_model.id),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(_) => Ok(()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
