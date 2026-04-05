use salvo::prelude::*;

use crate::infrastructure::http::handlers::auth::register_handler::register_handler;

pub fn router() -> Router {
    Router::new().post(register_handler)
}
