use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(StockProduct::Table, StockProduct::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(StockProduct::Table)
                    .col(big_integer(StockProduct::Id).primary_key())
                    .col(
                        ColumnDef::new(StockProduct::CompanyId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::Sku)
                            .string_len(60)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockProduct::Ean).string_len(60).null())
                    .col(ColumnDef::new(StockProduct::Barcode).string_len(60).null())
                    .col(
                        ColumnDef::new(StockProduct::Name)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::Brand)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::Category)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::Supplier)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::Shelf)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockProduct::Unit).string_len(40).not_null())
                    .col(ColumnDef::new(StockProduct::Stock).integer().not_null())
                    .col(
                        ColumnDef::new(StockProduct::Reserved)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockProduct::Minimum).integer().not_null())
                    .col(ColumnDef::new(StockProduct::Ideal).integer().not_null())
                    .col(ColumnDef::new(StockProduct::CostCents).integer().not_null())
                    .col(
                        ColumnDef::new(StockProduct::PriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::Status)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockProduct::Description).text().null())
                    .col(
                        ColumnDef::new(StockProduct::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockProduct::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockProduct::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "stock_products")]
pub enum StockProduct {
    Id,
    Table,
    CompanyId,
    Sku,
    Ean,
    Barcode,
    Name,
    Brand,
    Category,
    Supplier,
    Shelf,
    Unit,
    Stock,
    Reserved,
    Minimum,
    Ideal,
    CostCents,
    PriceCents,
    Status,
    Description,
    CreatedAt,
    UpdatedAt,
}
