use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "certification")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub company_id: i64,
    pub name: String,
    pub valid_until: Option<Date>,
    pub status_label: Option<String>,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
