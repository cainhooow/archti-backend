use salvo::prelude::*;

use crate::infrastructure::http::exceptions::HttpError;

#[handler]
pub async fn get_companies_handler() -> Result<(), HttpError> {
    Ok(())
}
