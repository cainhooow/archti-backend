use crate::domain::{
    builders::user_builder::UserBuilder, entities::user::User,
    repositories::user_repository_interface::UserRepository,
};
use std::sync::Arc;

use crate::application::{
    exceptions::{AppError, AppResult},
    ports::password_hasher::PasswordHasher,
};

pub struct CreateUserUseCase<U: UserRepository> {
    user_repository: U,
    hasher: Arc<dyn PasswordHasher>,
}

pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
}

impl<U: UserRepository> CreateUserUseCase<U> {
    pub fn new(user_repository: U, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self {
            user_repository,
            hasher,
        }
    }

    pub async fn execute(&self, command: CreateUserCommand) -> AppResult<User> {
        let hashed_password = self
            .hasher
            .hash(&command.password)
            .map_err(|err| AppError::EncryptionError(err.to_string()))?;

        let new_user = UserBuilder::new()
            .email(command.email)
            .full_name(command.full_name)
            .password_hash(hashed_password)
            .phone(command.phone)
            .build();

        let user = self.user_repository.save(&new_user).await?;

        Ok(user)
    }
}
