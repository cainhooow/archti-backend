#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PermissionDefinition {
    module: &'static str,
    action: &'static str,
    description: &'static str,
}

impl PermissionDefinition {
    pub const fn new(
        module: &'static str,
        action: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            module,
            action,
            description,
        }
    }

    pub fn code(&self) -> String {
        format!("{}.{}", self.module, self.action)
    }

    pub fn module(&self) -> &'static str {
        self.module
    }

    pub fn action(&self) -> &'static str {
        self.action
    }

    pub fn description(&self) -> &'static str {
        self.description
    }
}

pub mod catalog {
    use super::PermissionDefinition;

    pub const COMPANY_READ: PermissionDefinition =
        PermissionDefinition::new("company", "read", "Read company profile and settings");
    pub const COMPANY_MODIFY: PermissionDefinition =
        PermissionDefinition::new("company", "modify", "Modify company profile and settings");
    pub const COMPANY_MEMBERS_READ: PermissionDefinition =
        PermissionDefinition::new("company.members", "read", "Read company memberships");
    pub const COMPANY_MEMBERS_MANAGE: PermissionDefinition = PermissionDefinition::new(
        "company.members",
        "manage",
        "Invite, update and remove members",
    );
    pub const COMPANY_ROLES_READ: PermissionDefinition =
        PermissionDefinition::new("company.roles", "read", "Read company roles");
    pub const COMPANY_ROLES_MANAGE: PermissionDefinition = PermissionDefinition::new(
        "company.roles",
        "manage",
        "Create roles and assign permissions",
    );
    pub const BILLING_READ: PermissionDefinition =
        PermissionDefinition::new("billing", "read", "Read billing data");
    pub const BILLING_MANAGE: PermissionDefinition =
        PermissionDefinition::new("billing", "manage", "Manage billing data");
    pub const CLIENTS_READ: PermissionDefinition =
        PermissionDefinition::new("clients", "read", "Read client data");
    pub const CLIENTS_MANAGE: PermissionDefinition =
        PermissionDefinition::new("clients", "manage", "Create and update client data");
    pub const CLIENTS_PORTAL_READ: PermissionDefinition =
        PermissionDefinition::new("clients.portal", "read", "Access the client portal");
    pub const TECHNICIANS_READ: PermissionDefinition =
        PermissionDefinition::new("technicians", "read", "Read technician data");
    pub const TECHNICIANS_MANAGE: PermissionDefinition =
        PermissionDefinition::new("technicians", "manage", "Create and update technician data");
    pub const SERVICE_ORDERS_READ: PermissionDefinition =
        PermissionDefinition::new("service_orders", "read", "Read service orders");
    pub const SERVICE_ORDERS_CREATE: PermissionDefinition =
        PermissionDefinition::new("service_orders", "create", "Create service orders");
    pub const SERVICE_ORDERS_MANAGE: PermissionDefinition =
        PermissionDefinition::new("service_orders", "manage", "Update service orders");
    pub const SERVICE_ORDERS_UPDATE_STATUS: PermissionDefinition = PermissionDefinition::new(
        "service_orders",
        "update_status",
        "Update service order status",
    );
    pub const QUOTES_READ: PermissionDefinition =
        PermissionDefinition::new("quotes", "read", "Read quotes");
    pub const QUOTES_CREATE: PermissionDefinition =
        PermissionDefinition::new("quotes", "create", "Create quotes");
    pub const QUOTES_MANAGE: PermissionDefinition =
        PermissionDefinition::new("quotes", "manage", "Update quotes");
    pub const ORDERS_READ: PermissionDefinition =
        PermissionDefinition::new("orders", "read", "Read orders");
    pub const ORDERS_CREATE: PermissionDefinition =
        PermissionDefinition::new("orders", "create", "Create orders");
    pub const ORDERS_MANAGE: PermissionDefinition =
        PermissionDefinition::new("orders", "manage", "Update orders");
    pub const STOCK_READ: PermissionDefinition =
        PermissionDefinition::new("stock", "read", "Read stock data");
    pub const STOCK_MANAGE: PermissionDefinition =
        PermissionDefinition::new("stock", "manage", "Manage stock data");
    pub const RESOURCES_READ: PermissionDefinition =
        PermissionDefinition::new("resources", "read", "Read operational resources");
    pub const RESOURCES_MANAGE: PermissionDefinition =
        PermissionDefinition::new("resources", "manage", "Manage operational resources");

    pub const DEFAULT_PERMISSIONS: [PermissionDefinition; 27] = [
        COMPANY_READ,
        COMPANY_MODIFY,
        COMPANY_MEMBERS_READ,
        COMPANY_MEMBERS_MANAGE,
        COMPANY_ROLES_READ,
        COMPANY_ROLES_MANAGE,
        BILLING_READ,
        BILLING_MANAGE,
        CLIENTS_READ,
        CLIENTS_MANAGE,
        CLIENTS_PORTAL_READ,
        TECHNICIANS_READ,
        TECHNICIANS_MANAGE,
        SERVICE_ORDERS_READ,
        SERVICE_ORDERS_CREATE,
        SERVICE_ORDERS_MANAGE,
        SERVICE_ORDERS_UPDATE_STATUS,
        QUOTES_READ,
        QUOTES_CREATE,
        QUOTES_MANAGE,
        ORDERS_READ,
        ORDERS_CREATE,
        ORDERS_MANAGE,
        STOCK_READ,
        STOCK_MANAGE,
        RESOURCES_READ,
        RESOURCES_MANAGE,
    ];
}
