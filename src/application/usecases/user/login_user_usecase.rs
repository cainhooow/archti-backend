use crate::{
    application::{
        exceptions::{AppError, AppResult},
        ports::{
            password_hasher::PasswordHasher,
            token_service::{TokenOutput, TokenService},
        },
    },
    domain::{
        entities::user::User,
        repositories::user_repository_trait::{UserReadRepository, UserUpdateRepository},
    },
};

pub struct LoginUserUseCase<R, T, H>
where
    R: UserReadRepository + UserUpdateRepository,
    T: TokenService,
    H: PasswordHasher,
{
    repository: R,
    token_service: T,
    hasher: H,
}

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

pub struct LoginResponse {
    pub user: User,
    pub access_token: TokenOutput,
    pub refresh_token: TokenOutput,
}

impl<R, T, H> LoginUserUseCase<R, T, H>
where
    R: UserReadRepository + UserUpdateRepository,
    T: TokenService,
    H: PasswordHasher,
{
    pub fn new(repository: R, token_service: T, hasher: H) -> Self {
        Self {
            repository,
            token_service,
            hasher,
        }
    }

    pub async fn execute(&self, command: LoginUserCommand) -> AppResult<LoginResponse> {
        let mut user = self
            .repository
            .by_email(&command.email)
            .await
            .map_err(|_| AppError::AuthenticationFailed)?;

        if !self.hasher.verify(&command.password, user.password_hash()) {
            return Err(AppError::AuthenticationFailed);
        }

        user.record_login(chrono::Local::now().naive_local())
            .map_err(|_| AppError::Unexpected("Failed to record login".to_string()))?;
        let user = self.repository.update(&user).await?;

        let user_id = user.id().unwrap();
        let access_token = self
            .token_service
            .generate_access_token(user_id)
            .map_err(|_| AppError::Unexpected("Failed to generate access_token".to_string()))?;

        let refresh_token = self
            .token_service
            .generate_refresh_token(user_id)
            .map_err(|_| AppError::Unexpected("Failed to generate refresh_token".to_string()))?;

        Ok(LoginResponse {
            user,
            access_token,
            refresh_token,
        })
    }
}
