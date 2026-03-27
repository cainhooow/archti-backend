use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    application::usecases::user::login_user_usecase::LoginResponse,
    infrastructure::interfaces::http::resources::user_resource::UserResource,
};

#[derive(Serialize, Deserialize, Validate)]
pub struct AuthRequest {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 8))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[garde(email)]
    pub email: String,
}

#[derive(Serialize)]
pub struct AuthResource {
    pub user: UserResource,
    pub access_token: String,
    pub refresh_token: String,
}

impl From<LoginResponse> for AuthResource {
    fn from(value: LoginResponse) -> Self {
        Self {
            user: UserResource::from(value.user),
            access_token: value.access_token.token,
            refresh_token: value.refresh_token.token,
        }
    }
}
