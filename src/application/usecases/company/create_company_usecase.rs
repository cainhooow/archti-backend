use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::company::Company,
        repositories::company_repository_interface::CreateCompanyRepository,
    },
};

pub struct CreateCompanyUseCase<R>
where
    R: CreateCompanyRepository,
{
    repository: R,
}

pub struct CreateCompanyCommand {
    pub legal_name: String,
    pub trade_name: String,
    pub service_type: String,
    pub document: String,
    pub contact_name: String,
    pub primary_phone: String,
    pub secondary_phone: Option<String>,
    pub operational_base: String,
    pub notes: Option<String>,
}

impl<R> CreateCompanyUseCase<R>
where
    R: CreateCompanyRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCompanyCommand) -> AppResult<Company> {
        let company = Company::register(
            command.legal_name,
            command.trade_name,
            command.service_type,
            command.document,
            command.contact_name,
            command.primary_phone,
            command.secondary_phone,
            command.operational_base,
            command.notes,
        )?;

        let company = self.repository.create(&company).await?;
        Ok(company)
    }
}
