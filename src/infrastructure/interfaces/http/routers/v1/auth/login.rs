use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::auth::login_handler::login_handler;

pub fn router() -> Router {
    Router::with_path("/login").post(login_handler)
}
