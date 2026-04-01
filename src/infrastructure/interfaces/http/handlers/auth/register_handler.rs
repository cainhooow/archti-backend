use std::sync::Arc;

use garde::Validate;
use salvo::prelude::*;

use crate::{
    application::usecases::user::create_user_usecase::CreateUserCommand,
    infrastructure::{
        http::State,
        interfaces::http::{
            exceptions::HttpError,
            resources::{
                DataResponse,
                user_resources::{UserRequest, UserResource},
            },
        },
    },
};

#[handler]
pub async fn register_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> Result<(), HttpError> {
    let state = depot
        .obtain::<Arc<State>>()
        .map_err(|_| HttpError::InternalServerError("Failed to obtain app state".to_string()))?;

    match req.parse_body::<UserRequest>().await {
        Ok(validator) => {
            validator.validate()?;

            let user = state
                .identity
                .register(CreateUserCommand {
                    email: validator.email,
                    password: validator.password,
                    full_name: validator.full_name,
                    phone: validator.phone,
                })
                .await?;

            res.render(DataResponse::success(UserResource::from(user)));
            Ok(())
        }
        Err(err) => {
            return Err(HttpError::BadRequest(err.to_string()));
        }
    }
}
