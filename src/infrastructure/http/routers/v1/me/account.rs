use salvo::prelude::*;
pub mod change_password;

pub fn router() -> Router {
    Router::new().push(Router::with_path("change-password").push(change_password::router()))
}
