use tokio::sync::mpsc;

use crate::domain::{
    entities::user::User, repositories::user_repository_interface::CreateUserRepository,
};

use std::sync::Arc;

use crate::application::{
    events::IntegrationEvent,
    exceptions::{AppError, AppResult},
    ports::password_hasher::PasswordHasher,
};

pub struct CreateUserUseCase<U: CreateUserRepository> {
    user_repository: U,
    hasher: Arc<dyn PasswordHasher>,
    sender: mpsc::UnboundedSender<IntegrationEvent>,
}

pub struct CreateUserCommand {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone: Option<String>,
}

impl<U: CreateUserRepository> CreateUserUseCase<U> {
    pub fn new(
        user_repository: U,
        hasher: Arc<dyn PasswordHasher>,
        sender: mpsc::UnboundedSender<IntegrationEvent>,
    ) -> Self {
        Self {
            user_repository,
            hasher,
            sender,
        }
    }

    pub async fn execute(&self, command: CreateUserCommand) -> AppResult<User> {
        if self.user_repository.exists_by_email(&command.email).await? {
            return Err(AppError::Conflict("User already exists".to_string()));
        }

        let hashed_password = self
            .hasher
            .hash(&command.password)
            .map_err(|err| AppError::Unexpected(err.to_string()))?;

        let new_user = User::register(
            command.email,
            hashed_password,
            command.full_name,
            command.phone,
        )
        .map_err(|err| AppError::Unexpected(err.to_string()))?;

        let user = self.user_repository.create(&new_user).await?;
        self.sender
            .send(IntegrationEvent::WelcomeEmailRequested {
                email: new_user.email().to_string(),
                name: new_user.full_name().to_string(),
            })
            .ok();
        Ok(user)
    }
}
