use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlanVersionFeature::Table)
                    .col(uuid(PlanVersionFeature::PlanVersionId).not_null())
                    .col(
                        ColumnDef::new(PlanVersionFeature::FeatureId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-plan_version_feature")
                            .col(PlanVersionFeature::PlanVersionId)
                            .col(PlanVersionFeature::FeatureId)
                            .primary(),
                    )
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
        manager
            .drop_table(Table::drop().table(PlanVersionFeature::Table).to_owned())
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
