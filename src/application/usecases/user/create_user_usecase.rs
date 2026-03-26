use crate::domain::{
    entities::user::User,
    repositories::user_repository_interface::{CreateUserRepository},
};

use std::sync::Arc;

use crate::application::{
    exceptions::{AppError, AppResult},
    ports::password_hasher::PasswordHasher,
};

pub struct CreateUserUseCase<U: CreateUserRepository> {
    user_repository: U,
    hasher: Arc<dyn PasswordHasher>,
}

pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
}

impl<U: CreateUserRepository> CreateUserUseCase<U> {
    pub fn new(user_repository: U, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self {
            user_repository,
            hasher,
        }
    }

    pub async fn execute(&self, command: CreateUserCommand) -> AppResult<User> {
        if self.user_repository.exists_by_email(&command.email).await? {
            return Err(AppError::Bad("User already exists".to_string()));
        }

        let hashed_password = self
            .hasher
            .hash(&command.password)
            .map_err(|err| AppError::EncryptionError(err.to_string()))?;

        let new_user = User::register(
            command.email,
            hashed_password,
            command.full_name,
            command.phone,
        )
        .map_err(AppError::Bad)?;

        let user = self.user_repository.create(&new_user).await?;

        Ok(user)
    }
}
