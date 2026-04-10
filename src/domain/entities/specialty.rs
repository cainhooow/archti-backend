use chrono::NaiveDateTime;

use crate::domain::exceptions::DomainError;

pub struct Specialty {
    id: Option<i64>,
    company_id: i64,
    name: String,
    created_at: Option<NaiveDateTime>,
}

impl Specialty {
    pub fn create(company_id: i64, name: String) -> Result<Self, DomainError> {
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
        id: i64,
        company_id: i64,
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

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn company_id(&self) -> &i64 {
        &self.company_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }
}
