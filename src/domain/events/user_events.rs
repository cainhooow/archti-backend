pub struct PasswordForgotEvent {
    pub email: String,
    pub user_name: String,
}

pub struct PasswordResetEvent {
    pub email: String,
    pub user_name: String,
    pub reset_token: String,
}

pub struct PasswordChangedEvent {
    pub email: String,
    pub user_name: String,
}