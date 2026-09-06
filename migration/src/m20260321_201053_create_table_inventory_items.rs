use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company,
    m20260321_161049_create_table_stock_products::StockProduct,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(IventoryItem::Table, IventoryItem::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut stock_product_fk = ForeignKey::create()
            .from(IventoryItem::Table, IventoryItem::StockProductId)
            .to(StockProduct::Table, StockProduct::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(IventoryItem::Table)
                    .col(big_integer(IventoryItem::Id).primary_key())
                    .col(
                        ColumnDef::new(IventoryItem::CompanyId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::StockProductId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Code)
                            .string_len(60)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::AssetTag)
                            .string_len(60)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Name)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Category)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::SerialNumber)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::ItemCondition)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Status)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Location)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Owner)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::Origin)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(IventoryItem::AcquiredAt).date().null())
                    .col(ColumnDef::new(IventoryItem::WarrantyUntil).date().null())
                    .col(ColumnDef::new(IventoryItem::CostCents).integer().not_null())
                    .col(ColumnDef::new(IventoryItem::Notes).text().null())
                    .col(
                        ColumnDef::new(IventoryItem::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IventoryItem::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut stock_product_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IventoryItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "iventory_items")]
pub enum IventoryItem {
    Id,
    Table,
    CompanyId,
    StockProductId,
    Code,
    AssetTag,
    Name,
    Category,
    SerialNumber,
    ItemCondition,
    Status,
    Location,
    Owner,
    Origin,
    AcquiredAt,
    WarrantyUntil,
    CostCents,
    Notes,
    CreatedAt,
    UpdatedAt,
}
