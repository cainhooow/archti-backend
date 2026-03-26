use crate::domain::{entities::user::User, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait CreateUserRepository: Send + Sync {
    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError>;
    async fn create(&self, user: &User) -> Result<User, RepositoryError>;
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError>;
}
