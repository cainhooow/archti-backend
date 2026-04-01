use crate::application::notifications::EmailMessage;

pub struct WelcomeNotification {
    pub name: String,
}

impl EmailMessage for WelcomeNotification {
    fn template(&self) -> &str {
        "welcome"
    }

    fn subject(&self) -> &str {
        "Welcome to Archti!"
    }

    fn data(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
        })
    }
}
