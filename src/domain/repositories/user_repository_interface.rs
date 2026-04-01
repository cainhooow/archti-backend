use std::sync::Arc;

use crate::domain::{entities::user::User, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait CreateUserRepository: Send + Sync {
    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError>;
    async fn create(&self, user: &User) -> Result<User, RepositoryError>;
}

#[async_trait::async_trait]
pub trait UserReadRepository: Send + Sync {
    async fn first(&self) -> Result<User, RepositoryError>;
    async fn by_id(&self, id: &str) -> Result<User, RepositoryError>;
    async fn by_email(&self, email: &str) -> Result<User, RepositoryError>;
}

#[async_trait::async_trait]
pub trait UserUpdateRepository: Send + Sync {
    async fn update(&self, user: &User) -> Result<User, RepositoryError>;
}

#[async_trait::async_trait]
impl<T> CreateUserRepository for Arc<T>
where
    T: CreateUserRepository + ?Sized,
{
    async fn exists_by_email(&self, email: &str) -> Result<bool, RepositoryError> {
        (**self).exists_by_email(email).await
    }

    async fn create(&self, user: &User) -> Result<User, RepositoryError> {
        (**self).create(user).await
    }
}

#[async_trait::async_trait]
impl<T> UserReadRepository for Arc<T>
where
    T: UserReadRepository + ?Sized,
{
    async fn first(&self) -> Result<User, RepositoryError> {
        (**self).first().await
    }

    async fn by_id(&self, id: &str) -> Result<User, RepositoryError> {
        (**self).by_id(id).await
    }

    async fn by_email(&self, email: &str) -> Result<User, RepositoryError> {
        (**self).by_email(email).await
    }
}

#[async_trait::async_trait]
impl<T> UserUpdateRepository for Arc<T>
where
    T: UserUpdateRepository + ?Sized,
{
    async fn update(&self, user: &User) -> Result<User, RepositoryError> {
        (**self).update(user).await
    }
}
