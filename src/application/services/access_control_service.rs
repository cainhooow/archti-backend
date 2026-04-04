use crate::{
    application::exceptions::{AppError, AppResult},
    domain::repositories::{
        membership_repository_trait::MembershipRoleRepository,
        user_repository_trait::UserReadRepository,
    },
};

pub struct AuthorizeCompanyAccessCommand {
    pub user_id: String,
    pub company_id: String,
    pub permission_code: String,
}

pub struct AccessControlService<U, M>
where
    U: UserReadRepository + Clone,
    M: MembershipRoleRepository + Clone,
{
    user_repository: U,
    membership_repository: M,
}

impl<U, M> AccessControlService<U, M>
where
    U: UserReadRepository + Clone,
    M: MembershipRoleRepository + Clone,
{
    pub fn new(user_repository: U, membership_repository: M) -> Self {
        Self {
            user_repository,
            membership_repository,
        }
    }

    pub async fn authorize_company_access(
        &self,
        command: AuthorizeCompanyAccessCommand,
    ) -> AppResult<()> {
        let user = self.user_repository.by_id(&command.user_id).await?;

        // Tokens can outlive a status change, so authorization must re-check the user state.
        if !user.is_active() {
            return Err(AppError::AuthenticationFailed);
        }

        if user.is_super_admin() {
            return Ok(());
        }

        let allowed = self
            .membership_repository
            .has_permission(
                &command.company_id,
                &command.user_id,
                &command.permission_code,
            )
            .await?;

        if !allowed {
            return Err(AppError::PermissionDenied);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use crate::domain::{
        entities::user::{User, UserStatus},
        exceptions::RepositoryError,
        repositories::{
            membership_repository_trait::MembershipRoleRepository,
            user_repository_trait::UserReadRepository,
        },
    };

    use super::{AccessControlService, AuthorizeCompanyAccessCommand};

    #[derive(Clone)]
    struct FakeUserRepository {
        users: Arc<HashMap<String, User>>,
    }

    #[async_trait::async_trait]
    impl UserReadRepository for FakeUserRepository {
        async fn first(&self) -> Result<User, RepositoryError> {
            self.users
                .values()
                .next()
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }

        async fn by_id(&self, id: &str) -> Result<User, RepositoryError> {
            self.users.get(id).cloned().ok_or(RepositoryError::NotFound)
        }

        async fn by_email(&self, email: &str) -> Result<User, RepositoryError> {
            self.users
                .values()
                .find(|user| user.email() == email)
                .cloned()
                .ok_or(RepositoryError::NotFound)
        }
    }

    #[derive(Clone)]
    struct FakeMembershipRepository {
        allowed: bool,
    }

    #[async_trait::async_trait]
    impl MembershipRoleRepository for FakeMembershipRepository {
        async fn assign_role(
            &self,
            _membership_id: &str,
            _role_id: &str,
        ) -> Result<(), RepositoryError> {
            Ok(())
        }

        async fn has_permission(
            &self,
            _company_id: &str,
            _user_id: &str,
            _permission_code: &str,
        ) -> Result<bool, RepositoryError> {
            Ok(self.allowed)
        }
    }

    fn build_user(id: &str, status: UserStatus, is_super_admin: bool) -> User {
        User::restore(
            id.to_string(),
            format!("{id}@example.com"),
            "hashed".to_string(),
            "Example User".to_string(),
            None,
            status,
            is_super_admin,
            None,
            None,
            None,
            None,
        )
    }

    #[tokio::test]
    async fn allows_super_admin_without_membership_check() {
        let users = HashMap::from([(
            "user-1".to_string(),
            build_user("user-1", UserStatus::Active, true),
        )]);
        let service = AccessControlService::new(
            FakeUserRepository {
                users: Arc::new(users),
            },
            FakeMembershipRepository { allowed: false },
        );

        let result = service
            .authorize_company_access(AuthorizeCompanyAccessCommand {
                user_id: "user-1".to_string(),
                company_id: "company-1".to_string(),
                permission_code: "company.modify".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn rejects_inactive_user_even_if_membership_would_allow() {
        let users = HashMap::from([(
            "user-1".to_string(),
            build_user("user-1", UserStatus::Suspended, false),
        )]);
        let service = AccessControlService::new(
            FakeUserRepository {
                users: Arc::new(users),
            },
            FakeMembershipRepository { allowed: true },
        );

        let result = service
            .authorize_company_access(AuthorizeCompanyAccessCommand {
                user_id: "user-1".to_string(),
                company_id: "company-1".to_string(),
                permission_code: "company.modify".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn rejects_user_without_permission() {
        let users = HashMap::from([(
            "user-1".to_string(),
            build_user("user-1", UserStatus::Active, false),
        )]);
        let service = AccessControlService::new(
            FakeUserRepository {
                users: Arc::new(users),
            },
            FakeMembershipRepository { allowed: false },
        );

        let result = service
            .authorize_company_access(AuthorizeCompanyAccessCommand {
                user_id: "user-1".to_string(),
                company_id: "company-1".to_string(),
                permission_code: "company.modify".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}
