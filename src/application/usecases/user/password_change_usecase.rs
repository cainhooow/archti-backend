use tokio::sync::mpsc;

use crate::{
    application::{
        events::IntegrationEvent,
        exceptions::{AppError, AppResult},
        ports::password_hasher::PasswordHasher,
    },
    domain::repositories::user_repository_interface::{UserReadRepository, UserUpdateRepository},
};

pub struct ChangePasswordCommand {
    pub user_id: String,
    pub old_password: String,
    pub new_password: String,
}

pub struct ChangePasswordUseCase<R, H>
where
    R: UserUpdateRepository + UserReadRepository,
    H: PasswordHasher,
{
    pub repository: R,
    pub hasher: H,
    pub sender: mpsc::UnboundedSender<IntegrationEvent>,
}

impl<R, H> ChangePasswordUseCase<R, H>
where
    R: UserUpdateRepository + UserReadRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, hasher: H, sender: mpsc::UnboundedSender<IntegrationEvent>) -> Self {
        Self {
            repository,
            hasher,
            sender,
        }
    }

    pub async fn execute(&self, command: ChangePasswordCommand) -> AppResult<bool> {
        let mut user = self.repository.by_id(&command.user_id).await?;

        if !self
            .hasher
            .verify(&command.old_password, user.password_hash())
        {
            return Err(AppError::Unexpected(format!("Invalid password")));
        }

        if self
            .hasher
            .verify(&command.new_password, user.password_hash())
        {
            return Err(AppError::Unexpected(format!(
                "You cannot set a new password that is the same as the old one"
            )));
        }

        let new_hash = self.hasher.hash(&command.new_password)?;
        user.change_password(new_hash, chrono::Local::now().naive_local())?;
        user.record_last_password_change(chrono::Utc::now().naive_utc())?;

        self.repository
            .update(&user)
            .await
            .map_err(|_| AppError::Repository(format!("Failed to update password")))?;

        self.sender
            .send(IntegrationEvent::PasswordChangedEmailRequested {
                email: user.email().to_string(),
                name: user.full_name().to_string(),
            })
            .ok();

        Ok(true)
    }
}
