use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::me::account::change_password_handler::change_password_handler;

pub fn router() -> Router {
    Router::new().patch(change_password_handler)
}
