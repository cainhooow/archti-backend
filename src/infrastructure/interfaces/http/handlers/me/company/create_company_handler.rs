use std::sync::Arc;

use salvo::prelude::*;

use crate::{
    application::exceptions::{AppError, AppResult},
    infrastructure::http::State,
};

#[handler]
pub async fn create_company_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain app state")))?;
    
    Ok(())
}
