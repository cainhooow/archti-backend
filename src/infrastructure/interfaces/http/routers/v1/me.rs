use salvo::prelude::*;

// rustautomod
pub mod billing;
pub mod company;

pub fn router() -> Router {
    Router::with_path("/me")
}