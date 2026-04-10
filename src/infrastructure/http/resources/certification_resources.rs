use chrono::{NaiveDate, NaiveDateTime};
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::certification::Certification;

#[derive(Serialize, Deserialize)]
pub struct CertificationResource {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename(serialize = "validUntil"))]
    pub valid_until: Option<NaiveDate>,
    #[serde(rename(serialize = "statusLabel"))]
    pub status_label: Option<String>,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Validate)]
pub struct CertificationRequest {
    #[garde(length(min = 1))]
    pub name: String,
    #[serde(rename(serialize = "validUntil"))]
    #[garde(required)]
    pub valid_until: Option<NaiveDate>,
}

impl From<Certification> for CertificationResource {
    fn from(value: Certification) -> Self {
        Self {
            id: value.id().map(i64::to_string),
            name: value.name().to_string(),
            valid_until: value.valid_until(),
            status_label: value.status_label().map(str::to_string),
            created_at: value.created_at(),
        }
    }
}
