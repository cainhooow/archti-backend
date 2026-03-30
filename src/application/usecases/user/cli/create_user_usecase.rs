use crate::{
    application::exceptions::AppResult,
    domain::{entities::user::User, repositories::user_repository_interface::CreateUserRepository},
};

pub struct CreateUserCliCommand {
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub password: String,
    pub is_super_admin: bool,
}

pub struct CreateUserCliUseCase<R>
where
    R: CreateUserRepository,
{
    repository: R,
}

impl<R> CreateUserCliUseCase<R>
where
    R: CreateUserRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, command: CreateUserCliCommand) -> AppResult<User> {
        let mut user = User::register(
            command.email,
            command.password,
            command.full_name,
            command.phone,
        )?;

        if command.is_super_admin {
            user.set_admin(chrono::Local::now().naive_local())?;
        }

        let user = self.repository.create(&user).await?;
        Ok(user)
    }
}
