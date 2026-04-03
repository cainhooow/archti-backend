use crate::application::exceptions::AppResult;
use crate::domain::{
    entities::user::User, repositories::user_repository_trait::UserReadRepository,
};

pub struct FindUserById {
    pub id: String,
}

pub struct FindUserByIdQuery<R: UserReadRepository> {
    repository: R,
}

impl<R: UserReadRepository> FindUserByIdQuery<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: FindUserById) -> AppResult<User> {
        let user = self.repository.by_id(&query.id).await?;
        Ok(user)
    }
}
