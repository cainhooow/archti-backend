use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entities::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResource {
    pub id: Option<String>,
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub status_key: String,
    pub is_super_admin: bool,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<User> for UserResource {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            full_name: value.full_name,
            phone: value.phone,
            status_key: value.status_key,
            is_super_admin: value.is_super_admin,
            last_login_at: value.last_login_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<&User> for UserResource {
    fn from(value: &User) -> Self {
        Self {
            id: value.id.clone(),
            email: value.email.clone(),
            full_name: value.full_name.clone(),
            phone: value.phone.clone(),
            status_key: value.status_key.clone(),
            is_super_admin: value.is_super_admin,
            last_login_at: value.last_login_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}