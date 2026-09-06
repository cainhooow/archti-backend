use crate::domain::entities::user::{User as DomainUser, UserStatus};
use crate::infrastructure::models::user::Model as UserModel;

impl From<UserModel> for DomainUser {
    fn from(value: UserModel) -> Self {
        DomainUser::restore(
            value.id,
            value.email,
            value.password_hash,
            value.full_name,
            value.phone,
            UserStatus::try_from(value.status_key.as_str()).unwrap(),
            value.is_super_admin,
            value.last_login_at,
            value.last_password_changed_at,
            Some(value.created_at),
            Some(value.updated_at),
        )
    }
}
