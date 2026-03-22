use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_180536_create_table_company_subscriptions::CompanySubscription;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_subcription_fk = ForeignKey::create()
            .from(
                SubscriptionInvoice::Table,
                SubscriptionInvoice::CompanySubscriptionId,
            )
            .to(CompanySubscription::Table, CompanySubscription::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(SubscriptionInvoice::Table)
                    .col(uuid(SubscriptionInvoice::Id).primary_key())
                    .col(
                        ColumnDef::new(SubscriptionInvoice::CompanySubscriptionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::ReferenceLabel)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::StatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::AmountCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::PaidAmountCents)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::CurrencyCode)
                            .char_len(3)
                            .default("BRL")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::PeriodStart)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::PeriodEnd)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::IssuedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionInvoice::DueAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SubscriptionInvoice::PaidAt).timestamp())
                    .col(ColumnDef::new(SubscriptionInvoice::ProviderInvoiceRef).string_len(120))
                    .col(ColumnDef::new(SubscriptionInvoice::Notes).text().null())
                    .col(
                        ColumnDef::new(SubscriptionInvoice::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_subcription_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubscriptionInvoice::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "subscription_invoices")]
pub enum SubscriptionInvoice {
    Id,
    Table,
    CompanySubscriptionId,
    ReferenceLabel,
    StatusKey,
    AmountCents,
    PaidAmountCents,
    CurrencyCode,
    PeriodStart,
    PeriodEnd,
    IssuedAt,
    DueAt,
    PaidAt,
    ProviderInvoiceRef,
    Notes,
    CreatedAt,
}
