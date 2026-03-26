use crate::domain::entities::user::{User};
use crate::domain::exceptions::RepositoryError;
use crate::infrastructure::entities::user;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::repositories::user_repository_interface::UserRepository;

pub struct SeaOrmUserRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmUserRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn find_first(&self) -> Result<User, RepositoryError> {
        match user::Entity::find().limit(1).one(&*self.conn).await {
            Ok(Some(data)) => Ok(User::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError> {
        match user::Entity::find_by_id(Uuid::from_str(id).map_err(|_| RepositoryError::NotFound)?)
            .one(&*self.conn)
            .await
        {
            Ok(Some(data)) => Ok(User::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        match user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&*self.conn)
            .await
        {
            Ok(Some(data)) => Ok(User::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn create(&self, user: &User) -> Result<User, RepositoryError> {
        let new_user = user::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(user.email.clone()),
            full_name: Set(user.full_name.clone()),
            phone: Set(user.phone.clone()),
            status_key: Set(user.status_key.as_str().to_string().clone()),
            is_super_admin: Set(user.is_super_admin),
            password_hash: Set(user.password_hash.clone()),
            ..Default::default()
        };

        match new_user.insert(&*self.conn).await {
            Ok(data) => Ok(User::from(data)),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        match user::Entity::delete_by_id(Uuid::from_str(id).map_err(|_| RepositoryError::NotFound)?)
            .exec(&*self.conn)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn update(&self, user: &User) -> Result<User, RepositoryError> {
        let model = user::ActiveModel {
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            full_name: Set(user.full_name.clone()),
            phone: Set(user.phone.clone()),
            status_key: Set(user.status_key.as_str().to_string().clone()),
            updated_at: Set(chrono::Local::now().naive_local()),
            ..Default::default()
        };

        match model.update(&*self.conn).await {
            Ok(data) => Ok(User::from(data)),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }
}
