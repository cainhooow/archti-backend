use crate::{application::exceptions::AppResult, domain::{entities::user::{NewUser, User}, repositories::user_repository_interface::UserRepository}};


pub struct CreateUserUseCase<U: UserRepository> {
    user_repository: U
}

impl<U: UserRepository> CreateUserUseCase<U> {
    pub fn  new(user_repository: U) -> Self {
        Self {
            user_repository
        }
    }
    
    pub async fn execute(&self, new_user: &NewUser) -> AppResult<User> {
        let user = self.user_repository.save(new_user).await?;
        Ok(user)
    }
}