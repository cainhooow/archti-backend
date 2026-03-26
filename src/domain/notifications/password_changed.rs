use crate::domain::notifications::EmailMessage;

pub struct PasswordChangedMail {
    pub name: String,
}

impl EmailMessage for PasswordChangedMail {
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
