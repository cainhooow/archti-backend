use std::sync::Arc;

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::{
    entities::permission::Permission,
    exceptions::RepositoryError,
    repositories::permission_repository_trait::{
        PermissionCreateRepository, PermissionReadRepository,
    },
};

use crate::infrastructure::entities::permission;

pub struct SeaOrmPermissionRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmPermissionRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl PermissionCreateRepository for SeaOrmPermissionRepository {
    async fn create(&self, permission: &Permission) -> Result<Permission, RepositoryError> {
        let model = permission::ActiveModel {
            id: Set(Uuid::new_v4()),
            code: Set(permission.code().to_string()),
            module: Set(permission.module().to_string()),
            action: Set(permission.action().to_string()),
            description: Set(permission.description().map(str::to_string)),
            created_at: Set(permission.created_at()),
        };

        match model.insert(&*self.conn).await {
            Ok(model) => Ok(model.into()),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl PermissionReadRepository for SeaOrmPermissionRepository {
    async fn by_code(&self, code: &str) -> Result<Permission, RepositoryError> {
        match permission::Entity::find()
            .filter(permission::Column::Code.eq(code))
            .one(&*self.conn)
            .await
        {
            Ok(Some(model)) => Ok(model.into()),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn by_id(&self, id: &str) -> Result<Permission, RepositoryError> {
        let id = Uuid::parse_str(id).map_err(|err| RepositoryError::Generic(err.to_string()))?;

        match permission::Entity::find_by_id(id).one(&*self.conn).await {
            Ok(Some(model)) => Ok(model.into()),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
