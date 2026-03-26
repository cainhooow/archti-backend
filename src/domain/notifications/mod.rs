pub mod password_changed;
pub mod password_forgot;
pub mod password_reset;

pub trait EmailMessage: Send + Sync {
    fn template(&self) -> &str;
    fn subject(&self) -> &str;
    fn data(&self) -> serde_json::Value;
}
