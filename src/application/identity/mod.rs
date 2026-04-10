use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    application::{
        events::IntegrationEvent,
        exceptions::AppResult,
        ports::{
            password_hasher::PasswordHasher,
            password_reset_token_service::PasswordResetTokenService,
            token_service::{TokenOutput, TokenService},
        },
        usecases::user::{
            create_user_usecase::{CreateUserCommand, CreateUserUseCase},
            login_user_usecase::{LoginResponse, LoginUserCommand, LoginUserUseCase},
            password_change_usecase::{ChangePasswordCommand, ChangePasswordUseCase},
            password_forgot_usecase::{RequestPasswordResetCommand, RequestPasswordResetUseCase},
            password_reset_usecase::{PasswordResetCommand, PasswordResetUseCase},
        },
    },
    domain::{
        entities::user::User,
        repositories::user_repository_trait::{
            CreateUserRepository, UserReadRepository, UserUpdateRepository,
        },
    },
};

pub trait IdentityUserRepository:
    CreateUserRepository 
    + UserReadRepository
    + UserUpdateRepository {}

impl<T> IdentityUserRepository for T 
    where T: CreateUserRepository 
        + UserReadRepository 
        + UserUpdateRepository {}

pub struct RefreshSession {
    pub access_token: TokenOutput,
    pub refresh_token: TokenOutput,
}

pub struct IdentityApplication<R>
where
    R: IdentityUserRepository + Clone,
{
    user_repository: R,
    hasher: Arc<dyn PasswordHasher>,
    auth_service: Arc<dyn TokenService>,
    reset_token_service: Arc<dyn PasswordResetTokenService>,
    sender: mpsc::UnboundedSender<IntegrationEvent>,
    frontend_url: String,
}

impl<R> IdentityApplication<R>
where
    R: IdentityUserRepository + Clone,
{
    pub fn new(
        user_repository: R,
        hasher: Arc<dyn PasswordHasher>,
        auth_service: Arc<dyn TokenService>,
        reset_token_service: Arc<dyn PasswordResetTokenService>,
        sender: mpsc::UnboundedSender<IntegrationEvent>,
        frontend_url: String,
    ) -> Self {
        Self {
            user_repository,
            hasher,
            auth_service,
            reset_token_service,
            sender,
            frontend_url,
        }
    }

    pub async fn register(&self, command: CreateUserCommand) -> AppResult<User> {
        CreateUserUseCase::new(
            self.user_repository.clone(),
            self.hasher.clone(),
            self.sender.clone(),
        )
        .execute(command)
        .await
    }

    pub async fn login(&self, command: LoginUserCommand) -> AppResult<LoginResponse> {
        LoginUserUseCase::new(
            self.user_repository.clone(),
            self.auth_service.clone(),
            self.hasher.clone(),
        )
        .execute(command)
        .await
    }

    pub async fn request_password_reset(
        &self,
        command: RequestPasswordResetCommand,
    ) -> AppResult<()> {
        RequestPasswordResetUseCase::new(
            self.user_repository.clone(),
            self.reset_token_service.clone(),
            self.sender.clone(),
            self.frontend_url.clone(),
        )
        .execute(command)
        .await
    }

    pub async fn reset_password(&self, command: PasswordResetCommand) -> AppResult<bool> {
        PasswordResetUseCase::new(
            self.user_repository.clone(),
            self.reset_token_service.clone(),
            self.hasher.clone(),
            self.sender.clone(),
        )
        .execute(command)
        .await
    }

    pub async fn change_password(&self, command: ChangePasswordCommand) -> AppResult<bool> {
        ChangePasswordUseCase::new(
            self.user_repository.clone(),
            self.hasher.clone(),
            self.sender.clone(),
        )
        .execute(command)
        .await
    }

    pub async fn current_user(&self, user_id: i64) -> AppResult<User> {
        let user = self.user_repository.by_id(&user_id).await?;
        Ok(user)
    }

    pub async fn refresh_session(&self, refresh_token: String) -> AppResult<RefreshSession> {
        let user_id = self.auth_service.get_refresh_sub(&refresh_token)?;
        self.user_repository.by_id(&user_id).await?;

        let access_token = self.auth_service.renew_token(&refresh_token)?;
        let refresh_token = self.auth_service.generate_refresh_token(&user_id)?;

        Ok(RefreshSession {
            access_token,
            refresh_token,
        })
    }
}
