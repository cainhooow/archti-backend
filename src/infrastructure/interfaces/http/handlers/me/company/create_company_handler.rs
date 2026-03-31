use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::company::create_company_usecase::{CreateCompanyCommand, CreateCompanyUseCase},
    },
    infrastructure::{
        http::State,
        interfaces::http::resources::{
            DataResponse,
            company_resources::{CompanyRequest, CompanyResource},
        },
        persistence::sea_orm_company_repository::SeaOrmCompanyRepository,
    },
};

#[handler]
pub async fn create_company_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> AppResult<()> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| AppError::Unexpected(format!("Failed to obtain app state.")))?;

    let repository = SeaOrmCompanyRepository::new(state.db.clone());

    match req.parse_body::<CompanyRequest>().await {
        Ok(validator) => {
            _ = validator.validate()?;
            match CreateCompanyUseCase::new(repository)
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
                .await
            {
                Ok(company) => {
                    res.status_code(StatusCode::CREATED);
                    res.render(DataResponse::success(CompanyResource::from(company)));
                    Ok(())
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            return Err(AppError::Bad(e.to_string()));
        }
    }
}
