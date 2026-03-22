use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(Certification::Table, Certification::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Certification::Table)
                    .col(uuid(Certification::Id).primary_key())
                    .col(ColumnDef::new(Certification::CompanyId).uuid().not_null())
                    .col(
                        ColumnDef::new(Certification::Name)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Certification::ValidUntil).date().null())
                    .col(
                        ColumnDef::new(Certification::StatusLabel)
                            .string_len(80)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Certification::CreatedAt)
                            .timestamp()
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
            .drop_table(Table::drop().table(Certification::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "certifications")]
pub enum Certification {
    Id,
    Table,
    CompanyId,
    Name,
    ValidUntil,
    StatusLabel,
    CreatedAt,
}
