use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub status_key: String,
    pub is_super_admin: bool,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub status_key: String,
    pub is_super_admin: bool,
}

pub struct UpdateUser {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub status_key: Option<String>,
    pub is_super_admin: Option<bool>,
    pub last_login_at: Option<NaiveDateTime>,
}