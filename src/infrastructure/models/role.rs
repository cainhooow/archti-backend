use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub company_id: i64,
    pub code: String,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub is_system_role: bool,
    pub created_at: DateTime,

    #[sea_orm(belongs_to, from = "company_id", to = "id")]
    pub company: HasOne<super::company::Entity>,
    #[sea_orm(has_many, via = "role_permission")]
    pub permissions: HasMany<super::permission::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
