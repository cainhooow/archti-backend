#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainEvent {
    UserRegistered { user_id: String },
    PasswordResetRequested { user_id: String },
    PasswordChanged { user_id: String },
}
