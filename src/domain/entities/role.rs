use chrono::NaiveDateTime;

pub struct Role {
    pub id: Option<String>,
    pub company_id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_system_role: bool,
    pub created_at: NaiveDateTime,
}

impl Role {
    pub fn create(
        company_id: String,
        code: String,
        name: String,
        description: Option<String>,
        is_system_role: bool,
        created_at: NaiveDateTime,
    ) -> Result<Self, String> {
        if company_id.is_empty() {
            return Err("Company ID cannot be empty".to_string());
        }

        if code.is_empty() {
            return Err("Code cannot be empty".to_string());
        }
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if let Some(desc) = &description {
            if desc.is_empty() {
                return Err("Description cannot be empty".to_string());
            }
        }

        Ok(Self {
            id: None,
            company_id,
            code,
            name,
            description,
            is_system_role,
            created_at,
        })
    }

    pub fn restore(
        id: String,
        company_id: String,
        code: String,
        name: String,
        description: Option<String>,
        is_system_role: bool,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: Some(id),
            company_id,
            code,
            name,
            description,
            is_system_role,
            created_at,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn company_id(&self) -> &str {
        &self.company_id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn is_system_role(&self) -> bool {
        self.is_system_role
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}
