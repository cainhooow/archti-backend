use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
