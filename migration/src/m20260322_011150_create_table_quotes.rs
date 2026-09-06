use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company, m20260321_195712_create_table_clients::Client,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(Quote::Table, Quote::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut client_fk = ForeignKey::create()
            .from(Quote::Table, Quote::ClientId)
            .to(Client::Table, Client::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .col(big_integer(Quote::Id).primary_key())
                    .col(ColumnDef::new(Quote::CompanyId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Quote::QuoteNumber)
                            .string_len(40)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Quote::ClientId).big_integer().null())
                    .col(
                        ColumnDef::new(Quote::CustomerNameSnapshot)
                            .string_len(160)
                            .null(),
                    )
                    .col(ColumnDef::new(Quote::Notes).text().null())
                    .col(ColumnDef::new(Quote::ValidUntil).date().null())
                    .col(ColumnDef::new(Quote::StatusKey).string_len(40).not_null())
                    .col(ColumnDef::new(Quote::DiscountMode).string_len(40).null())
                    .col(ColumnDef::new(Quote::DiscountValueCents).integer())
                    .col(ColumnDef::new(Quote::DiscountValuePercent).integer())
                    .col(
                        ColumnDef::new(Quote::SubtotalCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Quote::TotalCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Quote::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Quote::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut client_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Quote::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "quotes")]
pub enum Quote {
    Id,
    Table,
    CompanyId,
    QuoteNumber,
    ClientId,
    CustomerNameSnapshot,
    Notes,
    ValidUntil,
    StatusKey,
    DiscountMode,
    DiscountValueCents,
    DiscountValuePercent,
    SubtotalCents,
    TotalCents,
    CreatedAt,
    UpdatedAt,
}
