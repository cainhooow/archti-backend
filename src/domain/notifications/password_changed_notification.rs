use crate::domain::notifications::EmailMessage;

pub struct PasswordChangedNotification {
    pub name: String,
}

impl EmailMessage for PasswordChangedNotification {
    fn template(&self) -> &str {
        "auth/password_changed"
    }

    fn subject(&self) -> &str {
        "Password Changed"
    }

    fn data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
        })
    }
}
