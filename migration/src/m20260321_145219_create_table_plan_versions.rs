use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .col(uuid(PlanVersion::Id).primary_key())
                    .col(ColumnDef::new(PlanVersion::PlanId).uuid().not_null())
                    .col(
                        ColumnDef::new(PlanVersion::VersionNumber)
                            .integer()
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
                    .col(ColumnDef::new(PlanVersion::TrialDays).integer().not_null())
                    .col(ColumnDef::new(PlanVersion::GraceDays).integer().not_null())
                    .col(
                        ColumnDef::new(PlanVersion::StatusKey)
                            .string_len(40)
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table("post").to_owned())
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
