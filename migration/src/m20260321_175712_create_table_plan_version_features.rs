use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_141334_create_table_plan_features::PlanFeature,
    m20260321_145219_create_table_plan_versions::PlanVersion,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut plan_version_fk = ForeignKey::create()
            .from(PlanVersionFeature::Table, PlanVersionFeature::PlanVersionId)
            .to(PlanVersion::Table, PlanVersion::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut feature_fk = ForeignKey::create()
            .from(PlanVersionFeature::Table, PlanVersionFeature::FeatureId)
            .to(PlanFeature::Table, PlanFeature::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

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
                    .foreign_key(&mut plan_version_fk)
                    .foreign_key(&mut feature_fk)
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
