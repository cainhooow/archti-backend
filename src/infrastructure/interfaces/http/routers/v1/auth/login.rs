use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::auth::login_handler::login_handler;

pub fn router() -> Router {
    Router::new().post(login_handler)
}
