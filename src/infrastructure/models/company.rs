use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "company")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub legal_name: String,
    pub trade_name: String,
    pub service_type: String,
    #[sea_orm(unique)]
    pub document: String,
    pub contact_name: String,
    pub primary_phone: String,
    pub secondary_phone: Option<String>,
    pub operational_base: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
