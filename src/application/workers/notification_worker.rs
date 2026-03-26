use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    application::handlers::NotificationHandler,
    domain::{
        events::DomainEvents,
        notifications::{
            password_changed_notification::PasswordChangedNotification,
            password_forgot_notification::PasswordForgotNotification,
            password_reset_notification::PasswordResetNotification,
            welcome_notification::WelcomeNotification,
        },
    },
};

pub async fn notification_worker(
    mut receiver: mpsc::UnboundedReceiver<DomainEvents>,
    handler: Arc<NotificationHandler>,
) {
    while let Some(event) = receiver.recv().await {
        match event {
            DomainEvents::UserRegistered { email, name } => {
                let msg = WelcomeNotification { name: name };
                let _ = handler.send(&email, msg).await;
            }
            DomainEvents::PasswordReset { email, name, link } => {
                let msg = PasswordResetNotification {
                    name: name,
                    link: link,
                };
                let _ = handler.send(&email, msg).await;
            }
            DomainEvents::PasswordForgot { email, name } => {
                let msg = PasswordForgotNotification { name: name };
                let _ = handler.send(&email, msg).await;
            }
            DomainEvents::PasswordChanged { email, name } => {
                let msg = PasswordChangedNotification { name: name };
                let _ = handler.send(&email, msg).await;
            }
        }
    }
}
