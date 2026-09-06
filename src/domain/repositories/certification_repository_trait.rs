use crate::domain::{entities::certification::Certification, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait CertificationCreateRepository {
    async fn create_certification(
        &self,
        certification: &Certification,
    ) -> Result<Certification, RepositoryError>;
}

#[async_trait::async_trait]
pub trait CertificationReadRepository {
    async fn all(&self) -> Result<Vec<Certification>, RepositoryError>;
    async fn by_id(&self, certification_id: &i64) -> Result<Certification, RepositoryError>;
    async fn by_status_label(
        &self,
        status_label: &str,
    ) -> Result<Vec<Certification>, RepositoryError>;
}

#[async_trait::async_trait]
pub trait CertificationUpdateRepository {
    async fn update_certification(
        &self,
        certification: &Certification,
    ) -> Result<Certification, RepositoryError>;
}
