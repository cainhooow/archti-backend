use chrono::NaiveDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MembershipStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MembershipType {
    Colaborator,
    Customer,
}

pub struct CompanyMembership {
    id: Option<String>,
    company_id: String,
    user_id: String,
    membership_type: MembershipType,
    status_key: MembershipStatus,
    display_name: Option<String>,
    invited_at: Option<NaiveDateTime>,
    accepted_at: Option<NaiveDateTime>,
    last_seen_at: Option<NaiveDateTime>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl MembershipStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MembershipStatus::Active => "active",
            MembershipStatus::Inactive => "inactive",
        }
    }
}

impl MembershipType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MembershipType::Colaborator => "colaborator",
            MembershipType::Customer => "customer",
        }
    }
}

impl TryFrom<&str> for MembershipStatus {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "active" => Ok(MembershipStatus::Active),
            "inactive" => Ok(MembershipStatus::Inactive),
            _ => Err(format!("Invalid membership status: {}", s)),
        }
    }
}

impl TryFrom<&str> for MembershipType {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "colaborator" => Ok(MembershipType::Colaborator),
            "customer" => Ok(MembershipType::Customer),
            _ => Err(format!("Invalid membership type: {}", s)),
        }
    }
}

impl CompanyMembership {
    pub fn register(
        company_id: String,
        user_id: String,
        membership_type: MembershipType,
        status_key: MembershipStatus,
        display_name: String,
    ) -> Result<Self, String> {
        if company_id.is_empty() {
            return Err("company_id cannot be empty".to_string());
        }
        if user_id.is_empty() {
            return Err("user_id cannot be empty".to_string());
        }

        if display_name.is_empty() {
            return Err("display_name cannot be empty".to_string());
        }

        Ok(Self {
            id: None,
            company_id,
            user_id,
            membership_type,
            status_key,
            display_name: Some(display_name),
            invited_at: None,
            accepted_at: None,
            last_seen_at: None,
            created_at: None,
            updated_at: None,
        })
    }

    pub fn restore(
        id: String,
        company_id: String,
        user_id: String,
        membership_type: MembershipType,
        status_key: MembershipStatus,
        display_name: Option<String>,
        invited_at: Option<NaiveDateTime>,
        accepted_at: Option<NaiveDateTime>,
        last_seen_at: Option<NaiveDateTime>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id: Some(id),
            company_id,
            user_id,
            membership_type,
            status_key,
            display_name,
            invited_at,
            accepted_at,
            last_seen_at,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn company_id(&self) -> &str {
        &self.company_id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn membership_type(&self) -> &MembershipType {
        &self.membership_type
    }

    // pub fn assign_role(&mut self, role: MembershipRole) -> Result<bool, String> {}

    pub fn status(&self) -> &MembershipStatus {
        &self.status_key
    }

    pub fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    pub fn invited_at(&self) -> Option<NaiveDateTime> {
        self.invited_at
    }

    pub fn accepted_at(&self) -> Option<NaiveDateTime> {
        self.accepted_at
    }

    pub fn last_seen_at(&self) -> Option<NaiveDateTime> {
        self.last_seen_at
    }

    pub fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
