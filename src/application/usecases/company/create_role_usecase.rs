use crate::{
    application::exceptions::AppResult,
    domain::repositories::role_repository_trait::RoleCreateRepository,
};

pub struct CreateCompanyRoleCommand {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
}

pub struct CreateCompanyRoleUseCase<C>
where
    C: RoleCreateRepository,
{
    repository: C,
}

impl<C> CreateCompanyRoleUseCase<C>
where
    C: RoleCreateRepository,
{
    pub fn new(repository: C) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateCompanyRoleCommand) -> AppResult<()> {
        
        Ok(())
    }
}
