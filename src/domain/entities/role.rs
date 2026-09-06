use chrono::NaiveDateTime;

pub const COMPANY_OWNER_CODE: &str = "company.owner";
pub const COMPANY_OWNER_NAME: &str = "Company Owner";
pub const COMPANY_OWNER_DESCRIPTION: &str =
    "System role automatically granted to the company creator";

#[derive(Debug, Clone)]
pub struct Role {
    pub id: Option<i64>,
    pub company_id: i64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_system_role: bool,
    pub created_at: NaiveDateTime,
}

impl Role {
    pub fn create(
        company_id: i64,
        code: String,
        name: String,
        description: Option<String>,
        is_system_role: bool,
        created_at: NaiveDateTime,
    ) -> Result<Self, String> {
        let code = code.trim().to_string();
        let name = name.trim().to_string();
        let description = description.map(|value| value.trim().to_string());

        if code.is_empty() {
            return Err("Code cannot be empty".to_string());
        }
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        if description.is_some() && description.as_ref().unwrap().is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        if !is_valid_role_code(&code) {
            return Err(
                "Role code must use lowercase letters, numbers, dots, or underscores".to_string(),
            );
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
        id: i64,
        company_id: i64,
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

    pub fn id(&self) -> Option<&i64> {
        self.id.as_ref()
    }

    pub fn company_id(&self) -> &i64 {
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

fn is_valid_role_code(value: &str) -> bool {
    let mut chars = value.chars();

    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => return false,
    }

    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '_')
}

#[cfg(test)]
mod tests {
    use super::Role;

    #[test]
    fn rejects_role_code_with_uppercase_letters() {
        let error = Role::create(
            1000030,
            "Company.Owner".to_string(),
            "Owner".to_string(),
            None,
            true,
            chrono::Local::now().naive_local(),
        )
        .expect_err("role should be rejected");

        assert!(error.contains("Role code must use lowercase"));
    }
}
