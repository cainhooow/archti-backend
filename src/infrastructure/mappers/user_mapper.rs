use crate::domain::entities::user::User as DomainUser;
use crate::infrastructure::entities::user::Model as UserModel;

impl From<UserModel> for DomainUser {
    fn from(value: UserModel) -> Self {
        Self {
            id: Some(value.id.to_string()),
            email: value.email,
            password_hash: value.password_hash,
            full_name: value.full_name,
            phone: value.phone,
            status_key: value.status_key,
            is_super_admin: value.is_super_admin,
            last_login_at: value.last_login_at,
            created_at: Some(value.created_at),
            updated_at: Some(value.updated_at),
        }
    }
}
