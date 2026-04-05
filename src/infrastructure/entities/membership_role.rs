use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "membership_role")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub membership_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
