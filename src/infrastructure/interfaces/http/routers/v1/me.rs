// rustautomod
use salvo::prelude::*;

pub mod billing;
pub mod company;

pub fn router() -> Router {
    Router::new()
}
