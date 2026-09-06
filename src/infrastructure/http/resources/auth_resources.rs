use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::{
    application::usecases::user::login_user_usecase::LoginResponse,
    infrastructure::http::resources::user_resources::UserResource,
};

#[derive(Serialize, Deserialize, Validate)]
pub struct AuthRequest {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 8))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PasswordForgotRequest {
    #[garde(email)]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[garde(ascii, length(min = 8))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResource {
    pub user: UserResource,
    #[serde(rename(serialize = "accessToken"))]
    pub access_token: String,
    #[serde(rename(serialize = "refreshToken"))]
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
