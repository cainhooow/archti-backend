use crate::domain::entities::role::Role as DomainRole;
use crate::infrastructure::models::role::Model as RoleModel;

impl From<RoleModel> for DomainRole {
    fn from(model: RoleModel) -> Self {
        DomainRole::restore(
            model.id.to_string(),
            model.company_id.to_string(),
            model.code,
            model.name,
            model.description,
            model.is_system_role,
            model.created_at,
        )
    }
}
