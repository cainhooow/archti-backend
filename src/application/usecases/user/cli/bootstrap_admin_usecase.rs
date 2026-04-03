use crate::{
    application::{
        exceptions::{AppError, AppResult},
        ports::password_hasher::PasswordHasher,
    },
    domain::{
        entities::user::User,
        repositories::user_repository_trait::{CreateUserRepository, UserReadRepository},
    },
};

pub struct BootstrapAdminCommand {
    pub email: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub password: String,
}

pub struct BootstrapAdminUseCase<R, H>
where
    R: CreateUserRepository + UserReadRepository,
    H: PasswordHasher,
{
    repository: R,
    hasher: H,
}

impl<R, H> BootstrapAdminUseCase<R, H>
where
    R: CreateUserRepository + UserReadRepository,
    H: PasswordHasher,
{
    pub fn new(repository: R, hasher: H) -> Self {
        Self { repository, hasher }
    }

    pub async fn execute(&self, command: BootstrapAdminCommand) -> AppResult<User> {
        if let Ok(_) = self.repository.by_email(&command.email).await {
            return Err(AppError::Conflict("User already exists".to_string()));
        }

        if let Ok(_) = self.repository.first().await {
            return Err(AppError::Unexpected(
                "Bootstrap can only be run once. After the superadmin account is created, this command cannot be run again.".to_string(),
            ));
        }

        let passwod_hash = self.hasher.hash(&command.password)?;

        let mut user = User::register(
            command.email,
            passwod_hash,
            command.full_name,
            command.phone,
        )?;

        user.set_admin(chrono::Local::now().naive_local())?;

        let user = self.repository.create(&user).await?;
        Ok(user)
    }
}
