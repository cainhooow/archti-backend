use crate::domain::{
    entities::role::Role, exceptions::RepositoryError,
    repositories::role_repository_trait::RoleCreateRepository,
};
use crate::infrastructure::entities::role;
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
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
        let model = role::ActiveModel {
            id: Set(Uuid::new_v4()),
            company_id: Set(Uuid::parse_str(role.company_id()).unwrap()),
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
