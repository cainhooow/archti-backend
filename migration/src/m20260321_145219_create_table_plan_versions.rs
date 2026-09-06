use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_140601_create_table_plans::Plan;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut plan_fk = ForeignKey::create()
            .from(PlanVersion::Table, PlanVersion::PlanId)
            .to(Plan::Table, Plan::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(PlanVersion::Table)
                    .col(big_integer(PlanVersion::Id).primary_key())
                    .col(
                        ColumnDef::new(PlanVersion::PlanId)
                            .big_integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::VersionNumber)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::BillingPeriod)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::BasePriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlanVersion::IncludedSeats).integer().null())
                    .col(ColumnDef::new(PlanVersion::SeatPriceCents).integer().null())
                    .col(
                        ColumnDef::new(PlanVersion::TrialDays)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::GraceDays)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::StatusKey)
                            .string_len(40)
                            .default("active")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::EffectiveFrom)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::EffectiveUntil)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersion::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut plan_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlanVersion::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "plan_versions")]
pub enum PlanVersion {
    Id,
    Table,
    PlanId,
    VersionNumber,
    BillingPeriod,
    BasePriceCents,
    IncludedSeats,
    SeatPriceCents,
    TrialDays,
    GraceDays,
    StatusKey,
    EffectiveFrom,
    EffectiveUntil,
    CreatedAt,
    UpdatedAt,
}
