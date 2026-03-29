// rustautomod
use salvo::prelude::*;

use crate::infrastructure::http::middlewares::auth_middleware::AuthMiddleware;

pub mod account;
pub mod billing;
pub mod company;

pub fn router() -> Router {
    Router::new()
        .hoop(AuthMiddleware)
        .push(Router::with_path("account").push(account::router()))
}
