use chrono::NaiveDateTime;
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::specialty::Specialty;

#[derive(Serialize, Deserialize)]
pub struct SpecialtyResource {
    pub id: Option<String>,
    #[serde(rename(serialize = "name"))]
    pub name: String,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Validate)]
pub struct SpecialtyRequest {
    #[garde(length(min = 1))]
    pub name: String,
}

impl From<Specialty> for SpecialtyResource {
    fn from(value: Specialty) -> Self {
        Self {
            id: value.id().map(i64::to_string),
            name: value.name().to_string(),
            created_at: value.created_at(),
        }
    }
}
