use crate::domain::notifications::EmailMessage;

pub struct PasswordForgotNotification {
    pub name: String,
}

impl EmailMessage for PasswordForgotNotification {
    fn template(&self) -> &str {
        "auth/password_forgot"
    }

    fn subject(&self) -> &str {
        "Password Forgot"
    }

    fn data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
        })
    }
}
