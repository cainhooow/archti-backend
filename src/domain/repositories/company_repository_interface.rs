use crate::domain::{
    entities::company::Company, exceptions::RepositoryError, value_objects::document_vo::Document,
};

#[async_trait::async_trait]
pub trait CreateCompanyRepository: Send + Sync {
    async fn exists_by_document(&self, document: &Document) -> Result<bool, RepositoryError>;
    async fn create(&self, company: &Company) -> Result<Company, RepositoryError>;
}

#[async_trait::async_trait]
pub trait CompanyUpdateRepository: Send + Sync {
    async fn update(&self, company: &Company) -> Result<Company, RepositoryError>;
}

#[async_trait::async_trait]
pub trait CompanyReadRepository: Send + Sync {
    async fn all(&self) -> Result<Vec<Company>, RepositoryError>;
    async fn by_id(&self, id: &str) -> Result<Company, RepositoryError>;
    async fn by_document(&self, document: &Document) -> Result<Company, RepositoryError>;
}
