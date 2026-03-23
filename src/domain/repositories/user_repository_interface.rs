use crate::domain::{
    entities::user::{User, NewUser}, exceptions::RepositoryError,
};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_first(&self) -> Result<User, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<User, RepositoryError>;
    async fn save(&self, user: &NewUser) -> Result<User, RepositoryError>;
    async fn delete(&self, id: &str) -> Result<bool, RepositoryError>;
    async fn update(&self, user: &User) -> Result<User, RepositoryError>;
}