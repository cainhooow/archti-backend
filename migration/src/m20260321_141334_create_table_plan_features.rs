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
                    .col(ColumnDef::new(PlanFeature::Code).string_len(100).not_null())
                    .col(ColumnDef::new(PlanFeature::Name).string_len(120).not_null())
                    .col(
                        ColumnDef::new(PlanFeature::Module)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PlanFeature::Description).text().null())
                    .col(
                        ColumnDef::new(PlanFeature::CreatedAt)
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
