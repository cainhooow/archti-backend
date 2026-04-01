use std::sync::Arc;

use tokio::sync::mpsc;
use tracing::{error, info};

use crate::application::{
    events::IntegrationEvent,
    handlers::NotificationHandler,
    notifications::{
        password_changed_notification::PasswordChangedNotification,
        password_forgot_notification::PasswordForgotNotification,
        password_reset_notification::PasswordResetNotification,
        welcome_notification::WelcomeNotification,
    },
};

pub async fn notification_worker(
    mut receiver: mpsc::UnboundedReceiver<IntegrationEvent>,
    handler: Arc<NotificationHandler>,
) {
    while let Some(event) = receiver.recv().await {
        info!(?event, "notification worker received event");
        match event {
            IntegrationEvent::WelcomeEmailRequested { email, name } => {
                let msg = WelcomeNotification { name: name };
                if let Err(err) = handler.send(&email, msg).await {
                    error!(%email, %err, "failed to send welcome notification");
                }
            }
            IntegrationEvent::PasswordResetEmailRequested { email, name, link } => {
                let msg = PasswordResetNotification {
                    name: name,
                    link: link,
                };
                if let Err(err) = handler.send(&email, msg).await {
                    error!(%email, %err, "failed to send password reset notification");
                }
            }
            IntegrationEvent::PasswordForgotEmailRequested { email, name } => {
                let msg = PasswordForgotNotification { name: name };
                if let Err(err) = handler.send(&email, msg).await {
                    error!(%email, %err, "failed to send password forgot notification");
                }
            }
            IntegrationEvent::PasswordChangedEmailRequested { email, name } => {
                let msg = PasswordChangedNotification { name: name };
                if let Err(err) = handler.send(&email, msg).await {
                    error!(%email, %err, "failed to send password changed notification");
                }
            }
        }
    }
}
