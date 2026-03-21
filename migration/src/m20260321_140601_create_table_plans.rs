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
                    .col(uuid(Plan::Id).primary_key())
                    .col(ColumnDef::new(Plan::Code).string_len(60).not_null())
                    .col(ColumnDef::new(Plan::Name).string_len(120).not_null())
                    .col(ColumnDef::new(Plan::Description).text().null())
                    .col(ColumnDef::new(Plan::StatusKey).string_len(40).not_null())
                    .col(
                        ColumnDef::new(Plan::IsPublic)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Plan::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Plan::UpdatedAt)
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
