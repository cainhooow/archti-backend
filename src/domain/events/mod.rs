#[derive(Debug)]
pub enum DomainEvents {
    UserRegistered {
        email: String,
        name: String,
    },

    PasswordChanged {
        email: String,
        name: String,
    },
    PasswordForgot {
        email: String,
        name: String,
    },
    PasswordReset {
        email: String,
        name: String,
        link: String,
    },
}
