use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::auth::password_reset_handler::password_reset_handler;

pub fn router() -> Router {
    Router::new().post(password_reset_handler)
}
