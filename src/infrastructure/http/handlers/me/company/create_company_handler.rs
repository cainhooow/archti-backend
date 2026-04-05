use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::company::RegisterCompanyCommand,
    infrastructure::{
        http::{HttpState, middlewares::auth_middleware::DEPOT_KEY_ID},
        http::{
            exceptions::HttpError,
            resources::{
                DataResponse,
                company_resources::{CompanyRequest, CompanyResource},
            },
        },
    },
};

#[handler]
pub async fn create_company_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<HttpState>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    let user_id = depot
        .get::<String>(DEPOT_KEY_ID)
        .map_err(|_| HttpError::InternalServerError("Failed to obtain user id".to_string()))?;
    let user = state
        .app
        .identity
        .current_user(String::from(user_id))
        .await?;

    match req.parse_body::<CompanyRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let registry = state
                .app
                .company
                .register_company(RegisterCompanyCommand {
                    legal_name: validator.legal_name,
                    trade_name: validator.trade_name,
                    service_type: validator.service_type,
                    document: validator.document,
                    contact_name: validator.contact_name,
                    primary_phone: validator.primary_phone,
                    secondary_phone: validator.secondary_phone,
                    operational_base: validator.operational_base,
                    notes: validator.notes,
                    owner_id: user.id().map(str::to_string).unwrap(),
                    owner_display_name: user.full_name().to_string(),
                })
                .await?;

            res.status_code(StatusCode::CREATED);
            res.render(DataResponse::success(CompanyResource::from(
                registry.company,
            )));
            Ok(())
        }
        Err(e) => {
            return Err(HttpError::BadRequest(e.to_string()));
        }
    }
}
