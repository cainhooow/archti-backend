use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
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
        let code = code.trim().to_string();
        let module = module.trim().to_string();
        let action = action.trim().to_string();
        let description = description.map(|value| value.trim().to_string());

        if code.is_empty() || module.is_empty() || action.is_empty() {
            return Err("Code, module, and action must not be empty".to_string());
        }

        if description.is_some() && description.as_ref().unwrap().is_empty() {
            return Err("Description must not be empty".to_string());
        }

        if !is_valid_permission_part(&module) {
            return Err("Module must use lowercase letters, numbers, dots, or underscores".to_string());
        }

        if !is_valid_permission_part(&action) {
            return Err("Action must use lowercase letters, numbers, dots, or underscores".to_string());
        }

        let expected_code = format!("{}.{}", module, action);
        if code != expected_code {
            return Err(format!(
                "Permission code must match '<module>.<action>' exactly. Expected '{}'",
                expected_code
            ));
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

fn is_valid_permission_part(value: &str) -> bool {
    let mut chars = value.chars();

    match chars.next() {
        Some(first) if first.is_ascii_lowercase() => {}
        _ => return false,
    }

    chars.all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.' || ch == '_')
}

#[cfg(test)]
mod tests {
    use super::Permission;

    #[test]
    fn creates_permission_when_code_matches_module_and_action() {
        let permission = Permission::create(
            "company.members.manage".to_string(),
            "company.members".to_string(),
            "manage".to_string(),
            Some("Manage members".to_string()),
        )
        .expect("permission should be valid");

        assert_eq!(permission.code(), "company.members.manage");
    }

    #[test]
    fn rejects_permission_when_code_does_not_match_module_and_action() {
        let error = Permission::create(
            "company.modify".to_string(),
            "company.members".to_string(),
            "manage".to_string(),
            None,
        )
        .expect_err("permission should be rejected");

        assert!(error.contains("Expected 'company.members.manage'"));
    }

    #[test]
    fn rejects_permission_with_uppercase_module() {
        let error = Permission::create(
            "Company.members.manage".to_string(),
            "Company.members".to_string(),
            "manage".to_string(),
            None,
        )
        .expect_err("permission should be rejected");

        assert!(error.contains("Module must use lowercase"));
    }
}
