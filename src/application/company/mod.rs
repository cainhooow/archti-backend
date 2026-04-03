use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::company::create_company_usecase::{CreateCompanyCommand, CreateCompanyUseCase},
    },
    domain::{
        entities::{
            company::Company,
            company_membership::{CompanyMembership, MembershipStatus, MembershipType},
        },
        repositories::{
            company_repository_trait::{CompanyReadRepository, CreateCompanyRepository},
            membership_repository_trait::CreateMembershipRepository,
        },
    },
};

pub trait CompanyRepository: CreateCompanyRepository + CompanyReadRepository {}

pub trait MembershipRepository: CreateMembershipRepository {}

impl<T> MembershipRepository for T where T: CreateMembershipRepository {}

impl<T> CompanyRepository for T where T: CreateCompanyRepository + CompanyReadRepository {}

pub struct CompanyApplication<R, S>
where
    R: CompanyRepository + Clone,
    S: MembershipRepository + Clone,
{
    company_reposistory: R,
    membership_repository: S,
}

pub struct RegisterCompanyCommand {
    pub legal_name: String,
    pub trade_name: String,
    pub service_type: String,
    pub document: String,
    pub contact_name: String,
    pub primary_phone: String,
    pub secondary_phone: Option<String>,
    pub operational_base: String,
    pub notes: Option<String>,
    pub owner_id: String,
    pub owner_display_name: String,
}

pub struct RegisterCompanyResult {
    pub company: Company,
    pub owner_membership: CompanyMembership,
}

impl<R, S> CompanyApplication<R, S>
where
    R: CompanyRepository + Clone,
    S: MembershipRepository + Clone,
{
    pub fn new(company_reposistory: R, membership_repository: S) -> Self {
        Self {
            company_reposistory,
            membership_repository,
        }
    }

    pub async fn register_company(
        &self,
        command: RegisterCompanyCommand,
    ) -> AppResult<RegisterCompanyResult> {
        let company = CreateCompanyUseCase::new(self.company_reposistory.clone())
            .execute(CreateCompanyCommand {
                legal_name: command.legal_name,
                trade_name: command.trade_name,
                service_type: command.service_type,
                document: command.document,
                contact_name: command.contact_name,
                primary_phone: command.primary_phone,
                secondary_phone: command.secondary_phone,
                operational_base: command.operational_base,
                notes: command.notes,
            })
            .await?;

        let company_id = company
            .id()
            .ok_or_else(|| AppError::Unexpected("Company created without id".to_string()))?
            .to_string();

        let membership = CompanyMembership::register(
            company_id,
            command.owner_id,
            MembershipType::Colaborator,
            MembershipStatus::Active,
            command.owner_display_name,
        )
        .map_err(AppError::RuleViolation)?;

        let owner_membership = self
            .membership_repository
            .create_membership(&membership)
            .await?;

        Ok(RegisterCompanyResult {
            company,
            owner_membership,
        })
    }
}
