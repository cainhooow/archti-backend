use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::auth::register_handler::register_handler;

pub fn router() -> Router {
    Router::with_path("register").post(register_handler)
}