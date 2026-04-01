use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::company::create_company_usecase::{
        CreateCompanyCommand, CreateCompanyUseCase,
    },
    infrastructure::{
        http::HttpState,
        interfaces::http::{
            exceptions::HttpError,
            resources::{
                DataResponse,
                company_resources::{CompanyRequest, CompanyResource},
            },
        },
        persistence::sea_orm_company_repository::SeaOrmCompanyRepository,
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
        .map_err(|_| HttpError::InternalServerError(format!("Failed to obtain app state.")))?;

    let repository = SeaOrmCompanyRepository::new(state.app.db.clone());

    match req.parse_body::<CompanyRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let company = CreateCompanyUseCase::new(repository)
                .execute(CreateCompanyCommand {
                    legal_name: validator.legal_name,
                    trade_name: validator.trade_name,
                    service_type: validator.service_type,
                    document: validator.document,
                    contact_name: validator.contact_name,
                    primary_phone: validator.primary_phone,
                    secondary_phone: validator.secondary_phone,
                    operational_base: validator.operational_base,
                    notes: validator.notes,
                })
                .await?;

            res.status_code(StatusCode::CREATED);
            res.render(DataResponse::success(CompanyResource::from(company)));
            Ok(())
        }
        Err(e) => {
            return Err(HttpError::BadRequest(e.to_string()));
        }
    }
}
