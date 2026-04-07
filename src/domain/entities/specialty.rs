use chrono::NaiveDateTime;

use crate::domain::exceptions::DomainError;

pub struct Specialty {
    id: Option<String>,
    company_id: String,
    name: String,
    created_at: Option<NaiveDateTime>,
}

impl Specialty {
    pub fn create(company_id: String, name: String) -> Result<Self, DomainError> {
        if company_id.is_empty() {
            return Err(DomainError::InvalidInput);
        }

        if name.is_empty() {
            return Err(DomainError::InvalidInput);
        }

        Ok(Self {
            id: None,
            company_id,
            name,
            created_at: None,
        })
    }

    pub fn restore(
        id: String,
        company_id: String,
        name: String,
        created_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id: Some(id),
            company_id,
            name,
            created_at,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn company_id(&self) -> &str {
        &self.company_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }
}
