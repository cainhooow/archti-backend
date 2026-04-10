use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company,
    m20260321_141334_create_table_plan_features::PlanFeature,
    m20260321_144012_create_table_company_memberships::CompanyMembership,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(
                CompanyFeatureOverride::Table,
                CompanyFeatureOverride::CompanyId,
            )
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut plan_feature_fk = ForeignKey::create()
            .from(
                CompanyFeatureOverride::Table,
                CompanyFeatureOverride::FeatureId,
            )
            .to(PlanFeature::Table, PlanFeature::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut membership_fk = ForeignKey::create()
            .from(
                CompanyFeatureOverride::Table,
                CompanyFeatureOverride::CreatedByMembershipId,
            )
            .to(CompanyMembership::Table, CompanyMembership::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanyFeatureOverride::Table)
                    .col(big_integer(CompanyFeatureOverride::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::CompanyId)
                            .big_integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::FeatureId)
                            .big_integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyFeatureOverride::CreatedByMembershipId)
                            .big_integer()
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
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut plan_feature_fk)
                    .foreign_key(&mut membership_fk)
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
