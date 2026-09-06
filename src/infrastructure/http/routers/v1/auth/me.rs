use salvo::prelude::*;

use crate::infrastructure::{
    http::handlers::auth::auth_me_handler::auth_me_handler,
    http::middlewares::auth_middleware::AuthMiddleware,
};

pub fn router() -> Router {
    Router::new().hoop(AuthMiddleware).get(auth_me_handler)
}
