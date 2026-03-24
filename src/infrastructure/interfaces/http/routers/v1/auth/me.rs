use salvo::prelude::*;

use crate::infrastructure::{
    http::middlewares::auth_middleware::AuthMiddleware,
    interfaces::http::handlers::auth::auth_me_handler::auth_me_handler,
};

pub fn router() -> Router {
    Router::with_path("/me")
        .hoop(AuthMiddleware)
        .get(auth_me_handler)
}
