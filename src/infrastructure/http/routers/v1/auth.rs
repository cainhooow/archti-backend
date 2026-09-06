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
    Router::new()
        .push(Router::with_path("register").push(register::router()))
        .push(Router::with_path("login").push(login::router()))
        .push(Router::with_path("logout").push(logout::router()))
        .push(Router::with_path("refresh-token").push(refresh_token::router()))
        .push(Router::with_path("forgot-password").push(forgot_password::router()))
        .push(Router::with_path("reset-password/{token}").push(reset_password::router()))
        .push(Router::with_path("me").push(me::router()))
}
