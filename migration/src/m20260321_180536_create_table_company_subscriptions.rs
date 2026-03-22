use sea_orm_migration::{prelude::*, schema::*, sea_orm::dynamic::Column};

use crate::{
    m20260321_041852_create_table_companies::Company,
    m20260321_145219_create_table_plan_versions::PlanVersion,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(CompanySubscription::Table, CompanySubscription::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut plan_version_fk = ForeignKey::create()
            .from(
                CompanySubscription::Table,
                CompanySubscription::PlanVersionId,
            )
            .to(PlanVersion::Table, PlanVersion::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanySubscription::Table)
                    .col(uuid(CompanySubscription::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanySubscription::CompanyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::PlanVersionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::StatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::BillingPeriod)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::PriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::SeatsIncluded)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::SeatPriceCents)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::AutoRenew)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::ProviderCustomerRef)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::ProviderCustomerSubscriptionRef)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::StartedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::TrialEndsAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::CurrentPeriodStart)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::CurrentPeriodEnd)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::RenewAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::CanceledAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::GraceEndsAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::CancelReason)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(CompanySubscription::Notes).text().null())
                    .col(
                        ColumnDef::new(CompanySubscription::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscription::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut plan_version_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CompanySubscription::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_subscriptions")]
pub enum CompanySubscription {
    Id,
    Table,
    CompanyId,
    PlanVersionId,
    StatusKey,
    BillingPeriod,
    PriceCents,
    SeatsIncluded,
    SeatPriceCents,
    AutoRenew,
    ProviderCustomerRef,
    ProviderCustomerSubscriptionRef,
    StartedAt,
    TrialEndsAt,
    CurrentPeriodStart,
    CurrentPeriodEnd,
    RenewAt,
    CanceledAt,
    GraceEndsAt,
    CancelReason,
    Notes,
    CreatedAt,
    UpdatedAt,
}
