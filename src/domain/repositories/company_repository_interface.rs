use std::sync::Arc;

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

#[async_trait::async_trait]
impl<T> CreateCompanyRepository for Arc<T>
where
    T: CreateCompanyRepository + ?Sized,
{
    async fn exists_by_document(&self, document: &Document) -> Result<bool, RepositoryError> {
        (**self).exists_by_document(document).await
    }

    async fn create(&self, company: &Company) -> Result<Company, RepositoryError> {
        (**self).create(company).await
    }
}

#[async_trait::async_trait]
impl<T> CompanyUpdateRepository for Arc<T>
where
    T: CompanyUpdateRepository + ?Sized,
{
    async fn update(&self, company: &Company) -> Result<Company, RepositoryError> {
        (**self).update(company).await
    }
}

#[async_trait::async_trait]
impl<T> CompanyReadRepository for Arc<T>
where
    T: CompanyReadRepository + ?Sized,
{
    async fn all(&self) -> Result<Vec<Company>, RepositoryError> {
        (**self).all().await
    }

    async fn by_id(&self, id: &str) -> Result<Company, RepositoryError> {
        (**self).by_id(id).await
    }

    async fn by_document(&self, document: &Document) -> Result<Company, RepositoryError> {
        (**self).by_document(document).await
    }
}
