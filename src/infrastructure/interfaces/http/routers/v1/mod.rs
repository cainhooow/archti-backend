pub use salvo::prelude::*;

// rustautomod
pub mod auth;
pub mod me;

pub fn router() -> Router {
    Router::with_path("/v1")
        .push(auth::router())
}