use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Quote::Table)
                    .col(uuid(Quote::Id).primary_key())
                    .col(ColumnDef::new(Quote::CompanyId).uuid().not_null())
                    .col(ColumnDef::new(Quote::QuoteNumber).string_len(40).not_null())
                    .col(ColumnDef::new(Quote::ClientId).uuid().null())
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
                    .col(ColumnDef::new(Quote::SubtotalCents).integer().not_null())
                    .col(ColumnDef::new(Quote::TotalCents).integer().not_null())
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("post").to_owned())
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
