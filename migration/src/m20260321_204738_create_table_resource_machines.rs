use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ResourceMachine::Table)
                    .col(uuid(ResourceMachine::Id).primary_key())
                    .col(
                        ColumnDef::new(ResourceMachine::ResourceId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ResourceMachine::Identifier)
                            .string_len(120)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ResourceMachine::Location)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ResourceMachine::Status)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ResourceMachine::Notes).text().null())
                    .col(
                        ColumnDef::new(ResourceMachine::CreatedAt)
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
            .drop_table(Table::drop().table(ResourceMachine::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "resource_machines")]
pub enum ResourceMachine {
    Id,
    Table,
    ResourceId,
    Identifier,
    Location,
    Status,
    Notes,
    CreatedAt,
}
