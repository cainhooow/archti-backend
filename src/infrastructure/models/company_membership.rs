use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "company_membership")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub company_id: i64,
    pub user_id: i64,
    pub membership_type: String,
    pub status_key: String,
    pub display_name: Option<String>,
    pub invited_at: Option<DateTime>,
    pub accepted_at: Option<DateTime>,
    pub last_seen_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,

    #[sea_orm(belongs_to, from = "company_id", to = "id")]
    pub company: Option<super::company::Entity>,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: Option<super::user::Entity>,
    #[sea_orm(has_many, via = "membership_role")]
    pub roles: HasMany<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
