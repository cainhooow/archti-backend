use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub company_id: i64,
    #[sea_orm(unique)]
    pub order_number: i64,
    pub quote_id: Option<i64>,
    pub client_id: Option<i64>,
    pub customer_name_snapshot: String,
    pub customer_profile_snapshot: Option<String>,
    pub customer_email_snapshot: Option<String>,
    pub customer_phone_snapshot: Option<String>,
    pub status_key: String,
    pub payment_status_key: Option<String>,
    pub channel_key: Option<String>,
    pub seller_name: Option<String>,
    pub delivery_mode: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub subtotal_cents: i32,
    pub freight_cents: i32,
    pub discount_cents: i32,
    pub total_cents: i32,
    pub created_at: DateTime,
    pub promised_window_at: Option<DateTime>,
    pub updated_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
