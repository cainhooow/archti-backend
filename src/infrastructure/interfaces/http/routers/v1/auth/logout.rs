use salvo::prelude::*;

use crate::infrastructure::{
    http::middlewares::auth_middleware::AuthMiddleware,
    interfaces::http::handlers::auth::auth_logout_handler::auth_logout_handler,
};

pub fn router() -> Router {
    Router::new()
        .hoop(AuthMiddleware)
        .get(auth_logout_handler)
}
