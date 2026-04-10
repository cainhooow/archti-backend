/*
* Base Company registration with
* Create company
* -> Base roles generated
* -> Default permissions for owner
* -> Roles and permissions assigned to owner
*/
use crate::{
    application::{
        exceptions::{AppError, AppResult},
        usecases::{
            company::{
                create_company_usecase::{CreateCompanyCommand, CreateCompanyUseCase},
                create_role_usecase::{CreateCompanyRoleCommand, CreateCompanyRoleUseCase},
            },
            system::bootstrap_permissions_usecase::BootstrapPermissionsUseCase,
        },
    },
    domain::{
        entities::{
            company::Company,
            company_membership::{CompanyMembership, MembershipStatus, MembershipType},
            permission_catalog::catalog::DEFAULT_PERMISSIONS,
            role::{COMPANY_OWNER_CODE, COMPANY_OWNER_DESCRIPTION, COMPANY_OWNER_NAME},
        },
        repositories::{
            company_repository_trait::{CompanyReadRepository, CreateCompanyRepository},
            membership_repository_trait::{CreateMembershipRepository, MembershipRoleRepository},
            permission_repository_trait::{PermissionCreateRepository, PermissionReadRepository},
            role_repository_trait::{
                RoleCreateRepository, RolePermissionRepository, RoleReadRepository,
            },
        },
    },
};

/**
 * Repository trait for company entities.
*/
pub trait CompanyRepository: CreateCompanyRepository + CompanyReadRepository {}
/**
 * Repository trait for membership entities.
 */
pub trait MembershipRepository: CreateMembershipRepository + MembershipRoleRepository {}
/**
 * Repository trait for role entities.
 */
pub trait RoleRepository:
    RoleCreateRepository + RoleReadRepository + RolePermissionRepository
{
}
/**
 * Repository trait for permission entities.
 */
pub trait PermissionRepository: PermissionCreateRepository + PermissionReadRepository {}

impl<T> MembershipRepository for T where T: CreateMembershipRepository + MembershipRoleRepository {}

impl<T> RoleRepository for T where
    T: RoleCreateRepository + RoleReadRepository + RolePermissionRepository
{
}

impl<T> PermissionRepository for T where T: PermissionCreateRepository + PermissionReadRepository {}

impl<T> CompanyRepository for T where T: CreateCompanyRepository + CompanyReadRepository {}

pub struct CompanyApplication<R, S, T, U>
where
    R: CompanyRepository + Clone,
    S: MembershipRepository + Clone,
    T: RoleRepository + Clone,
    U: PermissionRepository + Clone,
{
    company_reposistory: R,
    membership_repository: S,
    role_repository: T,
    permission_repository: U,
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
    pub owner_id: i64,
    pub owner_display_name: String,
}

pub struct RegisterCompanyResult {
    pub company: Company,
    pub owner_membership: CompanyMembership,
}

/**
* ApplicationService for registering a company.
*/
impl<R, S, T, U> CompanyApplication<R, S, T, U>
where
    R: CompanyRepository + Clone,
    S: MembershipRepository + Clone,
    T: RoleRepository + Clone,
    U: PermissionRepository + Clone,
{
    pub fn new(
        company_reposistory: R,
        membership_repository: S,
        role_repository: T,
        permission_repository: U,
    ) -> Self {
        Self {
            company_reposistory,
            membership_repository,
            role_repository,
            permission_repository,
        }
    }

    pub async fn register_company(
        &self,
        command: RegisterCompanyCommand,
    ) -> AppResult<RegisterCompanyResult> {
        // Company bootstrap relies on a stable global permission catalog.
        BootstrapPermissionsUseCase::new(self.permission_repository.clone())
            .execute()
            .await?;
        // Company created here
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
            .ok_or_else(|| AppError::Unexpected("Company created without id".to_string()))?;

        let membership = CompanyMembership::register(
            *company_id,
            command.owner_id,
            MembershipType::Colaborator,
            MembershipStatus::Active,
            command.owner_display_name,
        )
        .map_err(AppError::RuleViolation)?;

        // Owner membership created here
        let owner_membership = self
            .membership_repository
            .create_membership(&membership)
            .await?;

        // Owner role created here
        let owner_role = CreateCompanyRoleUseCase::new(self.role_repository.clone())
            .execute(CreateCompanyRoleCommand {
                company_id: *company_id,
                name: COMPANY_OWNER_NAME.to_string(),
                code: COMPANY_OWNER_CODE.to_string(),
                description: Some(COMPANY_OWNER_DESCRIPTION.to_string()),
                is_system_role: true,
            })
            .await?;

        let owner_role_id = owner_role
            .id()
            .ok_or_else(|| AppError::Unexpected("Owner role created without id".to_string()))?;
        let owner_membership_id = owner_membership
            .id()
            .ok_or_else(|| AppError::Unexpected("Membership created without id".to_string()))?;

        // The company creator must receive the full default catalog for the tenant owner role.
        for permission in DEFAULT_PERMISSIONS {
            self.role_repository
                .assign_permission(&owner_role_id, &permission.code())
                .await?;
        }

        // Owner role permissions assigned here
        self.membership_repository
            .assign_role(&owner_membership_id, &owner_role_id)
            .await?;

        Ok(RegisterCompanyResult {
            company,
            owner_membership,
        })
    }
}
