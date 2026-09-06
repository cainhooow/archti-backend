use salvo::prelude::*;

use crate::infrastructure::http::handlers::auth::auth_refresh_handler::auth_refresh_handler;

pub fn router() -> Router {
    Router::new().post(auth_refresh_handler)
}
