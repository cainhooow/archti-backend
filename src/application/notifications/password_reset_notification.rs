use crate::application::notifications::EmailMessage;

pub struct PasswordResetNotification {
    pub name: String,
    pub link: String,
}

impl EmailMessage for PasswordResetNotification {
    fn template(&self) -> &str {
        "auth/password_reset"
    }

    fn subject(&self) -> &str {
        "Password Reset"
    }

    fn data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "link": self.link,
        })
    }
}
