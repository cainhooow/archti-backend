use crate::domain::{
    entities::user::User, exceptions::RepositoryError,
    repositories::user_repository_interface::UserReadRepository,
};

pub struct FindUserByEmail {
    pub email: String,
}

pub struct FindUserByEmailQuery<R>
where
    R: UserReadRepository,
{
    repository: R,
}

impl<R> FindUserByEmailQuery<R>
where
    R: UserReadRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: FindUserByEmail) -> Result<User, RepositoryError> {
        let user = self.repository.by_email(&query.email).await?;
        Ok(user)
    }
}
