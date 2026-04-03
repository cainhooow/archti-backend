use tokio::sync::mpsc;

use crate::{
    application::{
        events::IntegrationEvent,
        exceptions::{AppError, AppResult},
        ports::password_reset_token_service::PasswordResetTokenService,
    },
    domain::repositories::user_repository_trait::UserReadRepository,
};

pub struct RequestPasswordResetCommand {
    pub email: String,
}

pub struct RequestPasswordResetUseCase<R, S>
where
    R: UserReadRepository,
    S: PasswordResetTokenService,
{
    pub repository: R,
    pub token_service: S,
    pub sender: mpsc::UnboundedSender<IntegrationEvent>,
    pub frontend_url: String,
}

impl<R, S> RequestPasswordResetUseCase<R, S>
where
    R: UserReadRepository,
    S: PasswordResetTokenService,
{
    pub fn new(
        repository: R,
        token_service: S,
        sender: mpsc::UnboundedSender<IntegrationEvent>,
        frontend_url: String,
    ) -> Self {
        Self {
            repository,
            token_service,
            sender,
            frontend_url,
        }
    }

    pub async fn execute(&self, command: RequestPasswordResetCommand) -> AppResult<()> {
        let user = match self.repository.by_email(&command.email).await {
            Ok(user) => user,
            Err(_) => return Ok(()),
        };

        let user_id = user
            .id()
            .ok_or_else(|| AppError::Unexpected("User without id".to_string()))?;

        let reset = self.token_service.generate_reset_token(user_id)?;
        let link = format!("{}/reset-password?token={}", self.frontend_url, reset.token);

        self.sender
            .send(IntegrationEvent::PasswordResetEmailRequested {
                email: user.email().to_string(),
                name: user.full_name().to_string(),
                link,
            })
            .map_err(|_| AppError::Unexpected(format!("Failed to enqueue password reset event")))?;

        Ok(())
    }
}
