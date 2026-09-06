use crate::domain::{entities::specialty::Specialty, exceptions::RepositoryError};

#[async_trait::async_trait]
pub trait SpecialtyCreateRepository {
    async fn create_specialty(&self, specialty: &Specialty) -> Result<Specialty, RepositoryError>;
}

#[async_trait::async_trait]
pub trait SpecialtyReadRepository {
    async fn all(&self) -> Result<Vec<Specialty>, RepositoryError>;
    async fn by_id(&self, specialty_id: &i64) -> Result<Specialty, RepositoryError>;
}

#[async_trait::async_trait]
pub trait SpecialtyUpdateRepository {
    async fn update_specialty(&self, specialty: &Specialty) -> Result<Specialty, RepositoryError>;
}
