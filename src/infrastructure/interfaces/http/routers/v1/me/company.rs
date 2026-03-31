use salvo::prelude::*;

use crate::infrastructure::interfaces::http::handlers::me::company::create_company_handler::create_company_handler;

pub fn router() -> Router {
    Router::new().post(create_company_handler)
}
