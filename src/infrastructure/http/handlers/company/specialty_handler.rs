use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::company::create_specialtie_usecase::{
        CreateSpecialtyCommand, CreateSpecialtyUseCase,
    },
    infrastructure::{
        http::{
            HttpState,
            exceptions::HttpError,
            resources::{
                DataResponse,
                specialty_resources::{SpecialtyRequest, SpecialtyResource},
            },
        },
        persistence::sea_orm_specialty_repository::SeaOrmSpecialtyRepository,
    },
};

#[handler]
pub async fn index_specialty_handler(_req: &mut Request, _depot: &mut Depot, _res: &mut Response) {}

#[handler]
pub async fn create_specialty_handler(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<HttpState>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state.".to_string()))?;

    let repository = SeaOrmSpecialtyRepository::new(state.app.db.clone());
    let company_id = depot
        .get::<i64>("id")
        .map_err(|_| HttpError::InternalServerError("Failed to obtain company id".to_string()))?;

    match req.parse_body::<SpecialtyRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let specialty = CreateSpecialtyUseCase::new(repository)
                .execute(CreateSpecialtyCommand {
                    company_id: *company_id,
                    name: validator.name,
                })
                .await?;

            res.status_code(StatusCode::CREATED);
            res.render(DataResponse::success(SpecialtyResource::from(specialty)));
        }
        Err(e) => {
            return Err(HttpError::BadRequest(e.to_string()));
        }
    }

    Ok(())
}
