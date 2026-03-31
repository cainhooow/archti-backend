use crate::{
    application::exceptions::{AppError, AppResult},
    domain::{
        entities::company::Company,
        repositories::company_repository_interface::{
            CompanyReadRepository, CreateCompanyRepository,
        },
        value_objects::document_vo::Document,
    },
};

pub struct CreateCompanyUseCase<R>
where
    R: CreateCompanyRepository + CompanyReadRepository,
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
    R: CreateCompanyRepository + CompanyReadRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCompanyCommand) -> AppResult<Company> {
        let document = Document::parse(command.document)?;

        if self.repository.exists_by_document(&document).await? {
            return Err(AppError::Bad(
                "There is already a company registered with this information.".to_string(),
            ));
        }

        let company = Company::register(
            command.legal_name,
            command.trade_name,
            command.service_type,
            document.to_string(),
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
