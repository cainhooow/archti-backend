// rustautomod
pub mod auth;
pub mod company;
pub mod me;

pub use salvo::prelude::*;

pub fn router() -> Router {
    Router::with_path("/v1")
        .push(Router::with_path("auth").push(auth::router()))
        .push(Router::with_path("me").push(me::router()))
        .push(Router::with_path("company").push(company::router()))
}
