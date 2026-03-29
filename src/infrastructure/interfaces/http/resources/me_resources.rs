use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[serde(rename(serialize = "oldPassword"))]
    #[garde(ascii)]
    pub old_password: String,
    #[serde(rename(serialize = "newPassword"))]
    #[garde(ascii)]
    pub new_password: String
}