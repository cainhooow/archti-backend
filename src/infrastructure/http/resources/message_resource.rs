use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageResource {
    pub message: String,
}
