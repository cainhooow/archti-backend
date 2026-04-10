use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::company::create_certification_usecase::{
        CreateCertificationCommand, CreateCertificationUseCase,
    },
    infrastructure::{
        http::{
            HttpState,
            exceptions::HttpError,
            resources::{
                DataResponse,
                certification_resources::{CertificationRequest, CertificationResource},
            },
        },
        persistence::sea_orm_certification_repository::SeaOrmCertificationRepository,
    },
};

#[handler]
pub async fn index_certification_handler(
    _req: &mut Request,
    _depot: &mut Depot,
    _res: &mut Response,
) {
}

#[handler]
pub async fn create_certification_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<HttpState>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain HttpState".to_string()))?;

    let repository = SeaOrmCertificationRepository::new(state.app.db.clone());

    let company_id: &str = depot
        .get::<String>("id")
        .map_err(|_| HttpError::InternalServerError("Failed to obtain company id".to_string()))?
        .as_ref();

    match req.parse_body::<CertificationRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let certification = CreateCertificationUseCase::new(repository)
                .execute(CreateCertificationCommand {
                    company_id: company_id.to_string(),
                    name: validator.name,
                    valid_until: validator.valid_until,
                })
                .await?;

            res.status_code(StatusCode::CREATED);
            res.render(DataResponse::success(CertificationResource::from(
                certification,
            )));

            Ok(())
        }
        Err(err) => {
            return Err(HttpError::BadRequest(err.to_string()));
        }
    }
}
