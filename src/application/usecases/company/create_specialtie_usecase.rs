use crate::{
    application::exceptions::AppResult,
    domain::{
        entities::specialty::Specialty,
        repositories::specialty_repository_trait::SpecialtyCreateRepository,
    },
};

pub struct CreateSpecialtyCommand {
    pub company_id: String,
    pub name: String,
}

pub struct CreateSpecialtyUseCase<R>
where
    R: SpecialtyCreateRepository,
{
    pub repository: R,
}

impl<R> CreateSpecialtyUseCase<R>
where
    R: SpecialtyCreateRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateSpecialtyCommand) -> AppResult<Specialty> {
        let specialty = Specialty::create(command.company_id, command.name)?;

        let specialty = self.repository.create_specialty(&specialty).await?;
        Ok(specialty)
    }
}
