use salvo::prelude::*;

use crate::infrastructure::{
    http::middlewares::auth_middleware::AuthMiddleware,
    interfaces::http::handlers::auth::auth_refresh_handler::auth_refresh_handler,
};

pub fn router() -> Router {
    Router::with_path("/refresh-token").post(auth_refresh_handler)
}
