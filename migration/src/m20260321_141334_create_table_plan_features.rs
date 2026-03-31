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
                    .table(PlanFeature::Table)
                    .col(uuid(PlanFeature::Id).primary_key())
                    .col(string_len(PlanFeature::Code, 100).unique_key().not_null())
                    .col(string_len(PlanFeature::Name, 120).not_null())
                    .col(string_len(PlanFeature::Module, 60).not_null())
                    .col(text(PlanFeature::Description).null())
                    .col(
                        timestamp(PlanFeature::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlanFeature::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "plan_features")]
pub enum PlanFeature {
    Id,
    Table,
    Code,
    Name,
    Module,
    Description,
    CreatedAt,
}
