use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "specialtie")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
