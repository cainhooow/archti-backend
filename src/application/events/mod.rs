#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationEvent {
    WelcomeEmailRequested {
        email: String,
        name: String,
    },
    PasswordChangedEmailRequested {
        email: String,
        name: String,
    },
    PasswordForgotEmailRequested {
        email: String,
        name: String,
    },
    PasswordResetEmailRequested {
        email: String,
        name: String,
        link: String,
    },
}
