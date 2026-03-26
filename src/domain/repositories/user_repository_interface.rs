use crate::domain::{entities::user::User, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait CreateUserRepository: Send + Sync {
    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError>;
    async fn create(&self, user: &User) -> Result<User, RepositoryError>;
}

#[async_trait::async_trait]
pub trait UserReadRepository: Send + Sync {
    async fn by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn by_email(&self, email: &str) -> Result<User, RepositoryError>;
}

#[async_trait::async_trait]
pub trait UserUpdateRepository: Send + Sync {
    async fn update(&self, user: &User) -> Result<User, RepositoryError>;
}