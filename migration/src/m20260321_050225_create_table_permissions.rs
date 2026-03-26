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
                    .table(Permission::Table)
                    .col(uuid(Permission::Id).primary_key())
                    .col(
                        ColumnDef::new(Permission::Code)
                            .string_len(100)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Permission::Module).string_len(60).not_null())
                    .col(ColumnDef::new(Permission::Action).string_len(60).not_null())
                    .col(ColumnDef::new(Permission::Description).text().null())
                    .col(
                        ColumnDef::new(Permission::CreatedAt)
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
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "permissions")]
pub enum Permission {
    Id,
    Table,
    Code,
    Module,
    Action,
    Description,
    CreatedAt,
}
