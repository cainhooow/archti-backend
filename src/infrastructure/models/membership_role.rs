use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "membership_role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub membership_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: i64,
    pub created_at: DateTime,

    #[sea_orm(belongs_to, from = "membership_id", to = "id")]
    pub membership: HasOne<super::company_membership::Entity>,
    #[sea_orm(belongs_to, from = "role_id", to = "id")]
    pub role: HasOne<super::role::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
