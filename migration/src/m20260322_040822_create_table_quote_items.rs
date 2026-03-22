use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_161049_create_table_stock_products::StockProduct,
    m20260322_011150_create_table_quotes::Quote,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut quote_fk = ForeignKey::create()
            .from(QuoteItem::Table, QuoteItem::QuoteId)
            .to(Quote::Table, Quote::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut stock_product_fk = ForeignKey::create()
            .from(QuoteItem::Table, QuoteItem::StockProductId)
            .to(StockProduct::Table, StockProduct::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(QuoteItem::Table)
                    .col(uuid(QuoteItem::Id).primary_key())
                    .col(ColumnDef::new(QuoteItem::QuoteId).uuid().not_null())
                    .col(ColumnDef::new(QuoteItem::StockProductId).uuid().null())
                    .col(ColumnDef::new(QuoteItem::SkuSnapshot).string_len(60).null())
                    .col(
                        ColumnDef::new(QuoteItem::ProductNameSnapshot)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(ColumnDef::new(QuoteItem::Quantity).integer().null())
                    .col(
                        ColumnDef::new(QuoteItem::UnitPriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(QuoteItem::SortOrder).integer().not_null())
                    .foreign_key(&mut quote_fk)
                    .foreign_key(&mut stock_product_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QuoteItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "quote_items")]
pub enum QuoteItem {
    Id,
    Table,
    QuoteId,
    StockProductId,
    SkuSnapshot,
    ProductNameSnapshot,
    Quantity,
    UnitPriceCents,
    SortOrder,
}
