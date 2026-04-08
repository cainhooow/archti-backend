use chrono::NaiveDate;

use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::certification::Certification,
        repositories::certification_repository_trait::CertificationCreateRepository,
    },
};

pub struct CreateCertificationCommand {
    pub company_id: String,
    pub name: String,
    pub valid_until: Option<NaiveDate>,
}

pub struct CreateCertificationUseCase<R>
where
    R: CertificationCreateRepository,
{
    pub repository: R,
}

impl<R> CreateCertificationUseCase<R>
where
    R: CertificationCreateRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCertificationCommand) -> AppResult<Certification> {
        let certification =
            Certification::create(command.company_id, command.name, command.valid_until, None)?;

        let certification = self.repository.create_certification(&certification).await?;
        Ok(certification)
    }
}
