use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .col(uuid(OrderItem::Id).primary_key())
                    .col(ColumnDef::new(OrderItem::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderItem::StockProductId).uuid().null())
                    .col(ColumnDef::new(OrderItem::SkuSnapshot).string_len(60).null())
                    .col(
                        ColumnDef::new(OrderItem::ProductNameSnapshot)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrderItem::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(OrderItem::UnitPriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrderItem::StatusKey).string_len(40).null())
                    .col(ColumnDef::new(OrderItem::SortOrder).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "order_items")]
pub enum OrderItem {
    Id,
    Table,
    OrderId,
    StockProductId,
    SkuSnapshot,
    ProductNameSnapshot,
    Quantity,
    UnitPriceCents,
    StatusKey,
    SortOrder,
}
