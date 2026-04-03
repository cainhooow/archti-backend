use tokio::sync::mpsc;

use crate::{
    application::{
        events::IntegrationEvent,
        exceptions::{AppError, AppResult},
        ports::{
            password_hasher::PasswordHasher,
            password_reset_token_service::PasswordResetTokenService,
        },
    },
    domain::repositories::user_repository_trait::{UserReadRepository, UserUpdateRepository},
};

pub struct PasswordResetCommand {
    pub token: String,
    pub password: String,
}

pub struct PasswordResetUseCase<R, S, H>
where
    R: UserReadRepository + UserUpdateRepository,
    S: PasswordResetTokenService,
    H: PasswordHasher,
{
    pub repository: R,
    pub token_service: S,
    pub hasher: H,
    pub sender: mpsc::UnboundedSender<IntegrationEvent>,
}

impl<R, S, H> PasswordResetUseCase<R, S, H>
where
    R: UserReadRepository + UserUpdateRepository,
    S: PasswordResetTokenService,
    H: PasswordHasher,
{
    pub fn new(
        repository: R,
        token_service: S,
        hasher: H,
        sender: mpsc::UnboundedSender<IntegrationEvent>,
    ) -> Self {
        Self {
            repository,
            token_service,
            hasher,
            sender,
        }
    }

    pub async fn execute(&self, command: PasswordResetCommand) -> AppResult<bool> {
        let user_id = self.token_service.verify_token(&command.token)?;
        let mut user = self
            .repository
            .by_id(&user_id)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        if let Some(last_password_change) = user.last_password_changed_at() {
            self.token_service
                .validate_token(&command.token, last_password_change)?;
        }

        let hashed_password = self.hasher.hash(&command.password)?;
        user.change_password(hashed_password, chrono::Local::now().naive_local())?;
        user.record_last_password_change(chrono::Utc::now().naive_utc())?;

        self.repository
            .update(&user)
            .await
            .map_err(|_| AppError::Unexpected("Failed to update user".to_string()))?;

        self.sender
            .send(IntegrationEvent::PasswordChangedEmailRequested {
                email: user.email().to_string(),
                name: user.full_name().to_string(),
            })
            .ok();

        Ok(true)
    }
}
