pub mod password_changed_notification;
pub mod password_forgot_notification;
pub mod password_reset_notification;
pub mod welcome_notification;

pub trait EmailMessage: Send + Sync {
    fn template(&self) -> &str;
    fn subject(&self) -> &str;
    fn data(&self) -> serde_json::Value;
}
