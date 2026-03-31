use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::value_objects::document_vo::Document;
use crate::domain::{
    entities::company::Company, exceptions::RepositoryError,
    repositories::company_repository_interface::CreateCompanyRepository,
};

use crate::infrastructure::entities::company;

pub struct SeaOrmCompanyRepository {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmCompanyRepository {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl CreateCompanyRepository for SeaOrmCompanyRepository {
    async fn exists_by_document(&self, document: &Document) -> Result<bool, RepositoryError> {
        match company::Entity::find_by_document(document.as_str())
            .one(&*self.conn)
            .await
        {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }

    async fn create(&self, company: &Company) -> Result<Company, RepositoryError> {
        let model = company::ActiveModel {
            id: Set(Uuid::new_v4()),
            legal_name: Set(company.legal_name().to_string()),
            trade_name: Set(company.trade_name().to_string()),
            service_type: Set(company.service_type().to_string()),
            document: Set(company.document().as_str().to_string()),
            contact_name: Set(company.contact_name().to_string()),
            primary_phone: Set(company.primary_phone().to_string()),
            secondary_phone: Set(company.secondary_phone().map(|p| p.to_string())),
            operational_base: Set(company.operational_base().to_string()),
            notes: Set(company.notes().map(|nt| nt.to_string())),
            ..Default::default()
        };

        match model.insert(&*self.conn).await {
            Ok(model) => Ok(model.try_into().map_err(RepositoryError::Generic)?),
            Err(err) => Err(RepositoryError::Generic(err.to_string())),
        }
    }
}
