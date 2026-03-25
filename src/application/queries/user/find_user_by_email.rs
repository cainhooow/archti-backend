// use crate::application::exceptions::AppResult;
// use crate::domain::{
//     entities::user::User, repositories::user_repository_interface::UserRepository,
// };

// pub struct FindUserByEmail {
//     pub email: String,
// }

// pub struct FindUserByEmailQuery<R: UserRepository> {
//     repository: R,
// }

// impl<R: UserRepository> FindUserByEmailQuery<R> {
//     pub fn new(repository: R) -> Self {
//         Self { repository }
//     }

//     pub async fn handle(&self, query: FindUserByEmail) -> AppResult<User> {
//         let user = self.repository.find_by_email(&query.email).await?;
//         Ok(user)
//     }
// }