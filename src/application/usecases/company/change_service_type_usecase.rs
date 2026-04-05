use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::company::Company,
        repositories::company_repository_trait::{CompanyReadRepository, CompanyUpdateRepository},
    },
};

pub struct ChangeServiceTypeUsecase<R>
where
    R: CompanyUpdateRepository + CompanyReadRepository,
{
    repository: R,
}

pub struct ChangeServiceTypeCommand {
    pub company_id: String,
    pub service_type: String,
}

impl<R> ChangeServiceTypeUsecase<R>
where
    R: CompanyUpdateRepository + CompanyReadRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: ChangeServiceTypeCommand) -> AppResult<Company> {
        let mut company = self.repository.by_id(&command.company_id).await?;
        company.change_service_type(command.service_type, chrono::Local::now().naive_utc())?;

        let updated_company = self.repository.update(&company).await?;
        Ok(updated_company)
    }
}
