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
                    .table(CompanyAddress::Table)
                    .col(uuid(CompanyAddress::Id).primary_key())
                    .col(ColumnDef::new(CompanyAddress::CompanyId).uuid().not_null())
                    .col(
                        ColumnDef::new(CompanyAddress::StreetLine)
                            .string_len(180)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::Street)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::District)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::City)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::State)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::ZipCode)
                            .string_len(16)
                            .not_null(),
                    )
                    .col(ColumnDef::new(CompanyAddress::Number).string_len(16).null())
                    .col(
                        ColumnDef::new(CompanyAddress::Complement)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::Reference)
                            .string_len(180)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::IsPrimary)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyAddress::CreatedAt)
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
            .drop_table(Table::drop().table(CompanyAddress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_addresses")]
pub enum CompanyAddress {
    Id,
    Table,
    CompanyId,
    StreetLine,
    Street,
    District,
    City,
    State,
    ZipCode,
    Number,
    Complement,
    Reference,
    IsPrimary,
    CreatedAt,
}
