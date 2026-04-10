use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use std::sync::Arc;

use crate::domain::{
    entities::certification::Certification, exceptions::RepositoryError,
    repositories::certification_repository_trait::CertificationCreateRepository,
};

use crate::infrastructure::models::certification;
use crate::infrastructure::services::snowflake_id::snowflake;

pub struct SeaOrmCertificationRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmCertificationRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CertificationCreateRepository for SeaOrmCertificationRepository {
    async fn create_certification(
        &self,
        certification: &Certification,
    ) -> Result<Certification, RepositoryError> {
        let model = certification::ActiveModel {
            id: Set(snowflake()),
            company_id: Set(*certification.company_id()),
            name: Set(certification.name().to_string()),
            valid_until: Set(certification.valid_until()),
            status_label: Set(certification.status_label().map(str::to_string)),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(data) => Ok(data.into()),
            Err(e) => Err(RepositoryError::Generic(e.to_string())),
        }
    }
}
