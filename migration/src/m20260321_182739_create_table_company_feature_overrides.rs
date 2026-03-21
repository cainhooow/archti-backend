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
                    .table(CompanyFeatureOverride::Table)
                    .col(uuid(CompanyFeatureOverride::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::CompanyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::FeatureId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::CreatedByMembershipId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::IsEnabled)
                            .boolean()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::LimitOverride)
                            .integer()
                            .null(),
                    )
                    .col(ColumnDef::new(CompanyFeatureOverride::Reason).text().null())
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::ExpiresAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::CreatedAt)
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
            .drop_table(
                Table::drop()
                    .table(CompanyFeatureOverride::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_feature_overrides")]
pub enum CompanyFeatureOverride {
    Id,
    Table,
    CompanyId,
    FeatureId,
    CreatedByMembershipId,
    IsEnabled,
    LimitOverride,
    Reason,
    ExpiresAt,
    CreatedAt,
}
