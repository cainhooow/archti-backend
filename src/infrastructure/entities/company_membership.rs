use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "company_membership")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub company_id: Uuid,
    #[sea_orm(unique)]
    pub user_id: Uuid,
    pub membership_type: String,
    pub status_key: String,
    pub display_name: Option<String>,
    pub invited_at: Option<DateTime>,
    pub accepted_at: Option<DateTime>,
    pub last_seen_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
