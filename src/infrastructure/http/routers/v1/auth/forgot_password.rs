use salvo::prelude::*;

use crate::infrastructure::http::handlers::auth::password_forgot_handler::forgot_password_handler;

pub fn router() -> Router {
    Router::new().post(forgot_password_handler)
}
