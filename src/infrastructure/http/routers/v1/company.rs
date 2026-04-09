use salvo::prelude::*;

use crate::infrastructure::http::{
    handlers::company::{
        certification_handler::{create_certification_handler, index_certification_handler},
        specialty_handler::{create_specialty_handler, index_specialty_handler},
    },
    middlewares::auth_middleware::AuthMiddleware,
};

pub fn router() -> Router {
    Router::with_path("{id}")
        .hoop(AuthMiddleware)
        .push(
            Router::with_path("specialties")
                .get(index_specialty_handler)
                .post(create_specialty_handler),
        )
        .push(
            Router::with_path("certifications")
                .get(index_certification_handler)
                .post(create_certification_handler),
        )
}
