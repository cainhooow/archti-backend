use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_161049_create_table_stock_products::StockProduct,
    m20260322_051337_create_table_orders::Order,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut order_fk = ForeignKey::create()
            .from(OrderItem::Table, OrderItem::OrderId)
            .to(Order::Table, Order::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut stock_product_fk = ForeignKey::create()
            .from(OrderItem::Table, OrderItem::StockProductId)
            .to(StockProduct::Table, StockProduct::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(OrderItem::Table)
                    .col(big_integer(OrderItem::Id).primary_key())
                    .col(ColumnDef::new(OrderItem::OrderId).big_integer().not_null())
                    .col(
                        ColumnDef::new(OrderItem::StockProductId)
                            .big_integer()
                            .null(),
                    )
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
                    .foreign_key(&mut order_fk)
                    .foreign_key(&mut stock_product_fk)
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
