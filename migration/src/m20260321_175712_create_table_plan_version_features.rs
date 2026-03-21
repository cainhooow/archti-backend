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
                    .table(PlanVersionFeature::Table)
                    .col(
                        uuid(PlanVersionFeature::PlanVersionId)
                            .primary_key()
                            .not_null(),
                    )
                    .col(uuid(PlanVersionFeature::FeatureId).primary_key().not_null())
                    .col(
                        ColumnDef::new(PlanVersionFeature::IsEnabled)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersionFeature::LimitValue)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersionFeature::LimitUnit)
                            .string_len(40)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PlanVersionFeature::CreatedAt)
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
#[sea_orm(table_name = "plan_version_features")]
pub enum PlanVersionFeature {
    Table,
    PlanVersionId,
    FeatureId,
    IsEnabled,
    LimitValue,
    LimitUnit,
    CreatedAt,
}
