use crate::infrastructure::models::{permission, role, role_permission};
use crate::{
    domain::{
        entities::role::Role,
        exceptions::RepositoryError,
        repositories::role_repository_trait::{
            RoleCreateRepository, RolePermissionRepository, RoleReadRepository,
        },
    },
    infrastructure::services::snowflake_id::snowflake,
};
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;

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
        let company_id = role.company_id();

        let model = role::ActiveModel {
            id: Set(snowflake()),
            company_id: Set(*company_id),
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
    async fn by_id(&self, id: &i64) -> Result<Role, RepositoryError> {
        match role::Entity::find_by_id(*id).one(&*self.conn).await {
            Ok(Some(data)) => Ok(Role::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn by_company_and_code(
        &self,
        company_id: &i64,
        code: &str,
    ) -> Result<Role, RepositoryError> {
        match role::Entity::find()
            .filter(role::Column::CompanyId.eq(*company_id))
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
        role_id: &i64,
        permission_code: &str,
    ) -> Result<(), RepositoryError> {
        let role = self.by_id(role_id).await?;

        let permission_model = match permission::Entity::find()
            .filter(permission::Column::Code.eq(permission_code))
            .one(&*self.conn)
            .await
        {
            Ok(Some(model)) => model,
            Ok(None) => return Err(RepositoryError::NotFound),
            Err(err) => return Err(RepositoryError::Generic(err.to_string())),
        };

        let existing = role_permission::Entity::find_by_id((*role_id, permission_model.id))
            .one(&*self.conn)
            .await
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        if existing.is_some() {
            return Ok(());
        }

        let model = role_permission::ActiveModel {
            role_id: Set(*role.id().unwrap()),
            permission_id: Set(permission_model.id),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(_) => Ok(()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
