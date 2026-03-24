use std::sync::Arc;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        ports::password_hasher::PasswordHasher,
    },
    domain::{
        entities::user::{NewUser, User},
        repositories::user_repository_interface::UserRepository,
    },
};

pub struct CreateUserUseCase<U: UserRepository> {
    user_repository: U,
    hasher: Arc<dyn PasswordHasher>,
}

impl<U: UserRepository> CreateUserUseCase<U> {
    pub fn new(user_repository: U, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self {
            user_repository,
            hasher,
        }
    }

    pub async fn execute(&self, new_user: &NewUser) -> AppResult<User> {
        let hashed_password = self
            .hasher
            .hash(&new_user.password)
            .map_err(|err| AppError::EncryptionError(err.to_string()))?;

        let user = self
            .user_repository
            .save(&NewUser {
                password: hashed_password,
                ..new_user.clone()
            })
            .await?;
        Ok(user)
    }
}
