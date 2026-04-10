use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_144012_create_table_company_memberships::CompanyMembership,
    m20260321_145219_create_table_plan_versions::PlanVersion,
    m20260321_180536_create_table_company_subscriptions::CompanySubscription,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_subscription_fk = ForeignKey::create()
            .from(
                CompanySubscriptionEvent::Table,
                CompanySubscriptionEvent::CompanySubscriptionId,
            )
            .to(CompanySubscription::Table, CompanySubscription::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut company_membership_fk = ForeignKey::create()
            .from(
                CompanySubscriptionEvent::Table,
                CompanySubscriptionEvent::CreatedByMembershipId,
            )
            .to(CompanyMembership::Table, CompanyMembership::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut from_plan_version_fk = ForeignKey::create()
            .from(
                CompanySubscriptionEvent::Table,
                CompanySubscriptionEvent::FromPlanVersionId,
            )
            .to(PlanVersion::Table, PlanVersion::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut to_plan_version_fk = ForeignKey::create()
            .from(
                CompanySubscriptionEvent::Table,
                CompanySubscriptionEvent::ToPlanVersionId,
            )
            .to(PlanVersion::Table, PlanVersion::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanySubscriptionEvent::Table)
                    .col(big_integer(CompanySubscriptionEvent::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::CompanySubscriptionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::CreatedByMembershipId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::EventType)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::FromPlanVersionId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::ToPlanVersionId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::FromStatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::ToStatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::Notes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::EffectiveAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_subscription_fk)
                    .foreign_key(&mut company_membership_fk)
                    .foreign_key(&mut from_plan_version_fk)
                    .foreign_key(&mut to_plan_version_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CompanySubscriptionEvent::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_subscription_events")]
pub enum CompanySubscriptionEvent {
    Id,
    Table,
    CompanySubscriptionId,
    CreatedByMembershipId,
    EventType,
    FromPlanVersionId,
    ToPlanVersionId,
    FromStatusKey,
    ToStatusKey,
    Notes,
    EffectiveAt,
    CreatedAt,
}
