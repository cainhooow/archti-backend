use crate::domain::notifications::EmailMessage;

pub struct PasswordForgotMail {
    pub name: String,
}

impl EmailMessage for PasswordForgotMail {
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
