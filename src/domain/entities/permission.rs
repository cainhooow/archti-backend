use chrono::NaiveDateTime;

pub struct Permission {
    id: Option<String>,
    code: String,
    module: String,
    action: String,
    description: Option<String>,
    created_at: NaiveDateTime,
}

impl Permission {
    pub fn create(
        code: String,
        module: String,
        action: String,
        description: Option<String>,
    ) -> Result<Self, String> {
        if code.is_empty() || module.is_empty() || action.is_empty() {
            return Err("Code, module, and action must not be empty".to_string());
        }

        if description.is_some() && description.as_ref().unwrap().is_empty() {
            return Err("Description must not be empty".to_string());
        }

        Ok(Self {
            id: None,
            code,
            module,
            action,
            description,
            created_at: chrono::Local::now().naive_local(),
        })
    }

    pub fn restore(
        id: String,
        code: String,
        module: String,
        action: String,
        description: Option<String>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: Some(id),
            code,
            module,
            action,
            description,
            created_at,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}
