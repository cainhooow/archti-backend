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
                    .table(Plan::Table)
                    .col(big_integer(Plan::Id).primary_key())
                    .col(string_len(Plan::Code, 60).unique_key().not_null())
                    .col(string_len(Plan::Name, 120).not_null())
                    .col(text(Plan::Description).null())
                    .col(string_len(Plan::StatusKey, 40).default("active").not_null())
                    .col(boolean(Plan::IsPublic).default(true).not_null())
                    .col(
                        timestamp(Plan::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Plan::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Plan::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "plans")]
pub enum Plan {
    Id,
    Table,
    Code,
    Name,
    Description,
    StatusKey,
    IsPublic,
    CreatedAt,
    UpdatedAt,
}
