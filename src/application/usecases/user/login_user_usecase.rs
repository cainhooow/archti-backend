use crate::{
    application::{
        exceptions::{AppError, AppResult},
        ports::{
            password_hasher::PasswordHasher,
            token_service::{TokenOutput, TokenService},
        },
    },
    domain::{entities::user::User, repositories::user_repository_interface::UserReadRepository},
};

pub struct LoginUserUseCase<R: UserReadRepository, T: TokenService, H: PasswordHasher> {
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

impl<R: UserReadRepository, T: TokenService, H: PasswordHasher> LoginUserUseCase<R, T, H> {
    pub fn new(repository: R, token_service: T, hasher: H) -> Self {
        Self {
            repository,
            token_service,
            hasher,
        }
    }

    pub async fn execute(&self, command: LoginUserCommand) -> AppResult<LoginResponse> {
        let user = self.repository.by_email(&command.email).await.map_err(|_| AppError::InvalidCredentials(
            format!("Password or email is incorrect")
        ))?;

        if !self.hasher.verify(&command.password, &user.password_hash()) {
            return Err(AppError::InvalidCredentials(format!(
                "Password or email is incorrect"
            )));
        }

        let user_id = user.id().unwrap();

        let access_token = self
            .token_service
            .generate_access_token(user_id)
            .map_err(|err| {
                AppError::Unexpected(format!(
                    "Failed to generate access_token: {}",
                    err.to_string()
                ))
            })?;
        
        let refresh_token = self
            .token_service
            .generate_refresh_token(user_id)
            .map_err(|err| {
                AppError::Unexpected(format!(
                    "Failed to generate refresh_token: {}",
                    err.to_string()
                ))
            })?;

        Ok(LoginResponse {
            user,
            access_token,
            refresh_token,
        })
    }
}
