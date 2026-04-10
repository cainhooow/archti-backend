use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_inrement = false)]
    pub id: i64,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<String>,
    pub status_key: String,
    pub is_super_admin: bool,
    pub last_login_at: Option<DateTime>,
    pub last_password_changed_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    #[sea_orm(has_many)]
    pub memberships: HasMany<super::company_membership::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
