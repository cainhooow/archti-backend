use crate::domain::entities::permission::Permission as DomainPermission;
use crate::infrastructure::models::permission::Model as PermissionModel;

impl From<PermissionModel> for DomainPermission {
    fn from(model: PermissionModel) -> Self {
        DomainPermission::restore(
            model.id.to_string(),
            model.code,
            model.module,
            model.action,
            model.description,
            model.created_at,
        )
    }
}
