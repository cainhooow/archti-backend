// rustautomod
use salvo::prelude::*;

pub mod forgot_password;
pub mod login;
pub mod logout;
pub mod me;
pub mod refresh_token;
pub mod register;
pub mod reset_password;

pub fn router() -> Router {
    Router::with_path("/auth")
        .push(register::router())
        .push(login::router())
}
