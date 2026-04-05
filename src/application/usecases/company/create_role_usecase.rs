use crate::{
    application::exceptions::{AppError, AppResult},
    domain::{
        entities::role::Role,
        exceptions::RepositoryError,
        repositories::role_repository_trait::{RoleCreateRepository, RoleReadRepository},
    },
};

pub struct CreateCompanyRoleCommand {
    pub company_id: String,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub is_system_role: bool,
}

pub struct CreateCompanyRoleUseCase<C>
where
    C: RoleCreateRepository + RoleReadRepository,
{
    repository: C,
}

impl<C> CreateCompanyRoleUseCase<C>
where
    C: RoleCreateRepository + RoleReadRepository,
{
    pub fn new(repository: C) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCompanyRoleCommand) -> AppResult<Role> {
        match self
            .repository
            .by_company_and_code(&command.company_id, &command.code)
            .await
        {
            Ok(role) => return Ok(role),
            Err(RepositoryError::NotFound) => {}
            Err(err) => return Err(err.into()),
        }

        let role = Role::create(
            command.company_id,
            command.code,
            command.name,
            command.description,
            command.is_system_role,
            chrono::Local::now().naive_local(),
        )
        .map_err(AppError::RuleViolation)?;

        let role = self.repository.create(&role).await?;
        Ok(role)
    }
}
