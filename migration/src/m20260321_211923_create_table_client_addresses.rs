use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClientAddress::Table)
                    .col(uuid(ClientAddress::Id).primary_key())
                    .col(ColumnDef::new(ClientAddress::ClientId).uuid().not_null())
                    .col(
                        ColumnDef::new(ClientAddress::Label)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::StreetLine)
                            .string_len(180)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::District)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::City)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::State)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::ZipCode)
                            .string_len(12)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ClientAddress::Number).string_len(16).null())
                    .col(
                        ColumnDef::new(ClientAddress::Complement)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::Reference)
                            .string_len(180)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::Usage)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::IsPrimary)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientAddress::CreatedAt)
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
            .drop_table(Table::drop().table(ClientAddress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "client_addresses")]
pub enum ClientAddress {
    Id,
    Table,
    ClientId,
    Label,
    StreetLine,
    District,
    City,
    State,
    ZipCode,
    Number,
    Complement,
    Reference,
    Usage,
    IsPrimary,
    CreatedAt,
}
