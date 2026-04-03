use crate::domain::{
    entities::user::User, exceptions::RepositoryError,
    repositories::user_repository_trait::UserReadRepository,
};

pub struct FindFirstUserQuery<R>
where
    R: UserReadRepository,
{
    repository: R,
}

impl<R> FindFirstUserQuery<R>
where
    R: UserReadRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self) -> Result<User, RepositoryError> {
        let user = self.repository.first().await?;
        Ok(user)
    }
}
