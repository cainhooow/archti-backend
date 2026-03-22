use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company,
    m20260321_195712_create_table_clients::Client, m20260322_011150_create_table_quotes::Quote,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(Order::Table, Order::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut quote_fk = ForeignKey::create()
            .from(Order::Table, Order::QuoteId)
            .to(Quote::Table, Quote::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut client_fk = ForeignKey::create()
            .from(Order::Table, Order::ClientId)
            .to(Client::Table, Client::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .col(uuid(Order::Id).primary_key())
                    .col(ColumnDef::new(Order::CompanyId).uuid().not_null())
                    .col(
                        ColumnDef::new(Order::OrderNumber)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::QuoteId).uuid().null())
                    .col(ColumnDef::new(Order::ClientId).uuid().null())
                    .col(
                        ColumnDef::new(Order::CustomerNameSnapshot)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Order::CustomerProfileSnapshot)
                            .string_len(80)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Order::CustomerEmailSnapshot)
                            .string_len(160)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Order::CustomerPhoneSnapshot)
                            .string_len(32)
                            .null(),
                    )
                    .col(ColumnDef::new(Order::StatusKey).string_len(40).not_null())
                    .col(
                        ColumnDef::new(Order::PaymentStatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::ChannelKey).string_len(40).not_null())
                    .col(ColumnDef::new(Order::SellerName).string_len(120).null())
                    .col(
                        ColumnDef::new(Order::DeliveryMode)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::Notes).text().null())
                    .col(
                        ColumnDef::new(Order::SubtotalCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Order::FreightCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Order::DiscountCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Order::TotalCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::PromisedWindowAt).timestamp().null())
                    .col(
                        ColumnDef::new(Order::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut quote_fk)
                    .foreign_key(&mut client_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "orders")]
pub enum Order {
    Id,
    Table,
    CompanyId,
    OrderNumber,
    QuoteId,
    ClientId,
    CustomerNameSnapshot,
    CustomerProfileSnapshot,
    CustomerEmailSnapshot,
    CustomerPhoneSnapshot,
    StatusKey,
    PaymentStatusKey,
    ChannelKey,
    SellerName,
    DeliveryMode,
    Notes,
    SubtotalCents,
    FreightCents,
    DiscountCents,
    TotalCents,
    CreatedAt,
    PromisedWindowAt,
    UpdatedAt,
}
