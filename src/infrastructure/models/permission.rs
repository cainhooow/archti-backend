use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "permission")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    #[sea_orm(unique)]
    pub code: String,
    pub module: String,
    pub action: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: DateTime,

    #[sea_orm(has_many, via = "role_permission")]
    pub roles: HasMany<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
