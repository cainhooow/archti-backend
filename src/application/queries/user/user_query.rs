use crate::{
    application::exceptions::AppResult,
    domain::{entities::user::User, repositories::user_repository_interface::UserRepository},
};

pub struct FindUserByEmailQuery<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> FindUserByEmailQuery<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, email: &str) -> AppResult<User> {
        let user = self.repository.find_by_email(email).await?;
        Ok(user)
    }
}

pub struct FindUserByIdQuery<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> FindUserByIdQuery<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: &str) -> AppResult<User> {
        let user = self.repository.find_by_id(id).await?;
        Ok(user)
    }
}