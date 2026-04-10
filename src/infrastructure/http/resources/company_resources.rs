use chrono::NaiveDateTime;
use garde::Validate;
use serde::{Deserialize, Serialize};

use crate::domain::entities::company::Company;

#[derive(Serialize, Deserialize, Validate)]
pub struct CompanyRequest {
    #[garde(length(min = 1, max = 160))]
    #[serde(rename(deserialize = "legalName"))]
    pub legal_name: String,
    #[garde(length(min = 1, max = 160))]
    #[serde(rename(deserialize = "tradeName"))]
    pub trade_name: String,
    #[garde(length(min = 1, max = 120))]
    #[serde(rename(deserialize = "serviceType"))]
    pub service_type: String,
    #[garde(length(min = 1, max = 32))]
    pub document: String,
    #[garde(length(min = 1, max = 120))]
    #[serde(rename(deserialize = "contactName"))]
    pub contact_name: String,
    #[garde(length(min = 1, max = 32))]
    #[serde(rename(deserialize = "phone"))]
    pub primary_phone: String,
    #[garde(length(min = 1, max = 32))]
    #[serde(rename(deserialize = "secondaryPhone"))]
    pub secondary_phone: Option<String>,
    #[garde(length(min = 1, max = 120))]
    #[serde(rename(deserialize = "operationalBase"))]
    pub operational_base: String,
    #[garde(length(min = 1))]
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyResource {
    pub id: Option<String>,
    #[serde(rename(serialize = "legalName"))]
    pub legal_name: String,
    #[serde(rename(serialize = "tradeName"))]
    pub trade_name: String,
    #[serde(rename(serialize = "serviceType"))]
    pub service_type: String,
    #[serde(rename(serialize = "document"))]
    pub document: String,
    #[serde(rename(serialize = "contactName"))]
    pub contact_name: String,
    #[serde(rename(serialize = "primaryPhone"))]
    pub primary_phone: String,
    #[serde(rename(serialize = "secondaryPhone"))]
    pub secondary_phone: Option<String>,
    #[serde(rename(serialize = "operationalBase"))]
    pub operational_base: String,
    #[serde(rename(serialize = "notes"))]
    pub notes: Option<String>,
    #[serde(rename(serialize = "createdAt"))]
    pub created_at: Option<NaiveDateTime>,
    #[serde(rename(serialize = "updatedAt"))]
    pub updated_at: Option<NaiveDateTime>,
}

impl From<Company> for CompanyResource {
    fn from(value: Company) -> Self {
        Self {
            id: value.id().map(i64::to_string),
            legal_name: value.legal_name().to_string(),
            trade_name: value.trade_name().to_string(),
            service_type: value.service_type().to_string(),
            document: value.document().to_string(),
            contact_name: value.contact_name().to_string(),
            primary_phone: value.primary_phone().to_string(),
            secondary_phone: value.secondary_phone().map(str::to_string),
            operational_base: value.operational_base().to_string(),
            notes: value.notes().map(str::to_string),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<&Company> for CompanyResource {
    fn from(value: &Company) -> Self {
        Self {
            id: value.id().map(i64::to_string),
            legal_name: value.legal_name().to_string(),
            trade_name: value.trade_name().to_string(),
            service_type: value.service_type().to_string(),
            document: value.document().to_string(),
            contact_name: value.contact_name().to_string(),
            primary_phone: value.primary_phone().to_string(),
            secondary_phone: value.secondary_phone().map(str::to_string),
            operational_base: value.operational_base().to_string(),
            notes: value.notes().map(str::to_string),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}
