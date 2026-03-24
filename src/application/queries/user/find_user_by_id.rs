use crate::{
    application::exceptions::AppResult,
    domain::{entities::user::User, repositories::user_repository_interface::UserRepository},
};

pub struct FindUserById {
    pub id: String,
}

pub struct FindUserByIdQuery<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> FindUserByIdQuery<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: FindUserById) -> AppResult<User> {
        let user = self.repository.find_by_id(&query.id).await?;
        Ok(user)
    }
}
