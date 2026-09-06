use crate::domain::entities::user::User;
use crate::domain::exceptions::RepositoryError;
use crate::domain::repositories::user_repository_trait::{
    CreateUserRepository, UserReadRepository, UserUpdateRepository,
};
use crate::infrastructure::models::user;
use crate::infrastructure::services::snowflake_id::snowflake;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;

pub struct SeaOrmUserRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmUserRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CreateUserRepository for SeaOrmUserRepository {
    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError> {
        match user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&*self.conn)
            .await
        {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn create(&self, user: &User) -> Result<User, RepositoryError> {
        let model = user::ActiveModel {
            id: Set(snowflake()),
            email: Set(user.email().to_string()),
            full_name: Set(user.full_name().to_string()),
            phone: Set(user.phone().map(|p| p.to_string())),
            status_key: Set(user.status().as_str().to_string()),
            is_super_admin: Set(user.is_super_admin()),
            password_hash: Set(user.password_hash().to_string()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(User::from(data)),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl UserUpdateRepository for SeaOrmUserRepository {
    async fn update(&self, user: &User) -> Result<User, RepositoryError> {
        let model = user::ActiveModel {
            id: Set(*user.id().unwrap()),
            email: Set(user.email().to_string()),
            full_name: Set(user.full_name().to_string()),
            phone: Set(user.phone().map(str::to_string)),
            status_key: Set(user.status().as_str().to_string()),
            is_super_admin: Set(user.is_super_admin()),
            password_hash: Set(user.password_hash().to_string()),
            last_login_at: Set(user.last_login_at()),
            last_password_changed_at: Set(user.last_password_changed_at()),
            ..Default::default()
        };

        match model.update(&*self.conn).await {
            Ok(data) => Ok(User::from(data)),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }
}

#[async_trait::async_trait]
impl UserReadRepository for SeaOrmUserRepository {
    async fn first(&self) -> Result<User, RepositoryError> {
        match user::Entity::find().one(&*self.conn).await {
            Ok(Some(data)) => Ok(User::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn by_id(&self, id: &i64) -> Result<User, RepositoryError> {
        match user::Entity::find_by_id(*id).one(&*self.conn).await {
            Ok(Some(data)) => Ok(User::from(data)),
            Ok(None) => Err(RepositoryError::NotFound),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }

    async fn by_email(&self, email: &str) -> Result<User, RepositoryError> {
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
}
