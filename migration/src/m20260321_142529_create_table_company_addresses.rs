use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(CompanyAddress::Table, CompanyAddress::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanyAddress::Table)
                    .col(uuid(CompanyAddress::Id).primary_key())
                    .col(uuid(CompanyAddress::CompanyId).not_null())
                    .col(string_len(CompanyAddress::StreetLine, 180).not_null())
                    .col(string_len(CompanyAddress::Street, 120).not_null())
                    .col(string_len(CompanyAddress::District, 120).not_null())
                    .col(string_len(CompanyAddress::City, 120).not_null())
                    .col(string_len(CompanyAddress::State, 32).not_null())
                    .col(string_len(CompanyAddress::ZipCode, 16).not_null())
                    .col(string_len(CompanyAddress::Number, 16).null())
                    .col(string_len(CompanyAddress::Complement, 120).null())
                    .col(string_len(CompanyAddress::Reference, 180).null())
                    .col(boolean(CompanyAddress::IsPrimary).default(true).not_null())
                    .col(
                        timestamp(CompanyAddress::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
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
