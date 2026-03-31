use garde::{Error, Validate};
use serde::{Deserialize, Serialize};

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