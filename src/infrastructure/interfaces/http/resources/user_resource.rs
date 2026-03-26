use chrono::NaiveDateTime;
use garde::Validate;
use serde::{Deserialize, Serialize};
use crate::domain::entities::user::User;

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRequest {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 8))]
    pub password: String,
    #[garde(ascii, length(min = 1))]
    #[serde(rename(deserialize = "fullName"))]
    pub full_name: String,
    #[garde(ascii, length(min = 1))]
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserResource {
    pub id: Option<String>,
    pub email: String,
    #[serde(rename(serialize = "fullName"))]
    pub full_name: String,
    pub phone: Option<String>,
    #[serde(rename(serialize = "status"))]
    pub status_key: String,
    #[serde(rename(serialize = "isSuperAdmin"))]
    pub is_super_admin: bool,
    #[serde(rename(serialize = "lastLoginAt"))]
    pub last_login_at: Option<NaiveDateTime>,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: Option<NaiveDateTime>,
}

impl From<User> for UserResource {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            full_name: value.full_name,
            phone: value.phone,
            status_key: value.status_key.as_str().to_string(),
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
            status_key: value.status_key.as_str().to_string().clone(),
            is_super_admin: value.is_super_admin,
            last_login_at: value.last_login_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
