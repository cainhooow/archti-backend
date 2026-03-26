use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    DeleteRequest,
}

impl UserStatus {
    pub fn as_str(&self) -> &str {
        match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
            UserStatus::DeleteRequest => "delete_request",
        }
    }
}

impl TryFrom<&str> for UserStatus {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "suspended" => Ok(UserStatus::Suspended),
            "delete_request" => Ok(UserStatus::DeleteRequest),
            _ => Err(format!("Invalid user status: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub status_key: UserStatus,
    pub is_super_admin: bool,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn register(
        email: String,
        password_hash: String,
        full_name: String,
        phone: Option<String>,
    ) -> Result<Self, String> {
        if full_name.trim().is_empty() {
            return Err("Full name cannot be empty".to_string());
        }

        Ok(Self {
            id: None,
            email,
            password_hash,
            full_name,
            phone,
            status_key: UserStatus::Inactive,
            is_super_admin: false,
            last_login_at: None,
            created_at: None,
            updated_at: None,
        })
    }

    pub fn activate(&mut self, now: NaiveDateTime) -> Result<(), String> {
        if self.status_key == UserStatus::Active {
            return Err("User is already active".to_string());
        }

        self.status_key = UserStatus::Active;
        self.updated_at = Some(now);
        Ok(())
    }

    pub fn suspend(&mut self, now: NaiveDateTime) -> Result<(), String> {
        if self.is_super_admin {
            return Err("Cannot suspend a super admin".to_string());
        }

        self.status_key = UserStatus::Suspended;
        self.updated_at = Some(now);
        Ok(())
    }

    pub fn change_password(&mut self, new_hash: String, now: NaiveDateTime) -> Result<(), String> {
        if new_hash.trim().is_empty() {
            return Err("Password hash cannot be empty".to_string());
        }

        self.password_hash = new_hash;
        self.updated_at = Some(now);
        Ok(())
    }

    pub fn record_login(&mut self, now: NaiveDateTime) -> Result<(), String> {
        if self.status_key != UserStatus::Active {
            return Err("Only active users can log in".to_string());
        }

        self.last_login_at = Some(now);
        self.updated_at = Some(now);
        Ok(())
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn status(&self) -> &UserStatus {
        &self.status_key
    }

    pub fn is_active(&self) -> bool {
        self.status_key == UserStatus::Active
    }
}
