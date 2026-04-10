use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_141334_create_table_plan_features::PlanFeature,
    m20260321_180536_create_table_company_subscriptions::CompanySubscription,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_subscription_fk = ForeignKey::create()
            .from(
                CompanySubscriptionAddon::Table,
                CompanySubscriptionAddon::CompanySubscriptionId,
            )
            .to(CompanySubscription::Table, CompanySubscription::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut plan_feature_fk = ForeignKey::create()
            .from(
                CompanySubscriptionAddon::Table,
                CompanySubscriptionAddon::FeatureId,
            )
            .to(PlanFeature::Table, PlanFeature::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanySubscriptionAddon::Table)
                    .col(big_integer(CompanySubscriptionAddon::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::CompanySubscriptionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::FeatureId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::Code)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::Name)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::AddonKind)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::UnitPriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::LimitIncrement)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::StatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::StartsAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::EndsAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionAddon::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_subscription_fk)
                    .foreign_key(&mut plan_feature_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CompanySubscriptionAddon::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_subscription_addons")]
pub enum CompanySubscriptionAddon {
    Id,
    Table,
    CompanySubscriptionId,
    FeatureId,
    Code,
    Name,
    AddonKind,
    Quantity,
    UnitPriceCents,
    LimitIncrement,
    StatusKey,
    StartsAt,
    EndsAt,
    CreatedAt,
}
