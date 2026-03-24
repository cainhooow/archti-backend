use crate::domain::entities::user::NewUser;

pub struct UserBuilder {
    email: String,
    password_hash: String,
    full_name: String,
    phone: Option<String>,
    status_key: String,
    is_super_admin: bool,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            email: String::new(),
            password_hash: String::new(),
            full_name: String::new(),
            phone: None,
            status_key: "INACTIVE".to_string(),
            is_super_admin: false,
        }
    }
}

impl UserBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn password_hash(mut self, password_hash: impl Into<String>) -> Self {
        self.password_hash = password_hash.into();
        self
    }

    pub fn full_name(mut self, full_name: impl Into<String>) -> Self {
        self.full_name = full_name.into();
        self
    }

    pub fn phone(mut self, phone: Option<String>) -> Self {
        self.phone = phone;
        self
    }

    pub fn status_key(mut self, status_key: impl Into<String>) -> Self {
        self.status_key = status_key.into();
        self
    }

    pub fn is_super_admin(mut self, is_super_admin: bool) -> Self {
        self.is_super_admin = is_super_admin;
        self
    }

    pub fn build(self) -> NewUser {
        NewUser {
            email: self.email,
            password: self.password_hash,
            full_name: self.full_name,
            phone: self.phone,
            status_key: self.status_key,
            is_super_admin: self.is_super_admin,
        }
    }
}
