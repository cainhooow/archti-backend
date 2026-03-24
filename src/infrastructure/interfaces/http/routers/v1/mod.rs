// rustautomod
pub mod auth;
pub mod me;

pub use salvo::prelude::*;

pub fn router() -> Router {
    Router::with_path("/v1")
        .push(auth::router())
}
