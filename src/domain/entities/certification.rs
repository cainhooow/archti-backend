use crate::domain::exceptions::DomainError;
use chrono::{NaiveDate, NaiveDateTime};

pub struct Certification {
    id: Option<i64>,
    company_id: i64,
    name: String,
    valid_until: Option<NaiveDate>,
    status_label: Option<String>,
    created_at: Option<NaiveDateTime>,
}

impl Certification {
    pub fn create(
        company_id: i64,
        name: String,
        valid_until: Option<NaiveDate>,
        status_label: Option<String>,
    ) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidInput);
        }

        if status_label.is_some() && status_label.as_ref().unwrap().is_empty() {
            return Err(DomainError::InvalidInput);
        }

        Ok(Self {
            id: None,
            company_id,
            name,
            valid_until,
            status_label,
            created_at: None,
        })
    }

    pub fn restore(
        id: i64,
        company_id: i64,
        name: String,
        valid_until: Option<NaiveDate>,
        status_label: Option<String>,
        created_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id: Some(id),
            company_id,
            name,
            valid_until,
            status_label,
            created_at,
        }
    }

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn company_id(&self) -> &i64 {
        &self.company_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn valid_until(&self) -> Option<NaiveDate> {
        self.valid_until
    }

    pub fn status_label(&self) -> Option<&str> {
        self.status_label.as_deref()
    }

    pub fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }
}
