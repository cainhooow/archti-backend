use crate::{
    domain::{
        entities::specialty::Specialty, exceptions::RepositoryError,
        repositories::specialty_repository_trait::SpecialtyCreateRepository,
    },
    infrastructure::models::specialty,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

pub struct SeaOrmSpecialtyRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmSpecialtyRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl SpecialtyCreateRepository for SeaOrmSpecialtyRepository {
    async fn create_specialty(&self, specialty: &Specialty) -> Result<Specialty, RepositoryError> {
        let company_id = Uuid::from_str(specialty.company_id())
            .map_err(|err| RepositoryError::Generic(err.to_string()))?;

        let model = specialty::ActiveModel {
            id: Set(Uuid::new_v4()),
            company_id: Set(company_id),
            name: Set(specialty.name().to_string()),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(data.into()),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
