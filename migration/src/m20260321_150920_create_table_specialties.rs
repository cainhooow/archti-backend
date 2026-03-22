use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(Specialtie::Table, Specialtie::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Specialtie::Table)
                    .col(uuid(Specialtie::Id).primary_key())
                    .col(ColumnDef::new(Specialtie::CompanyId).uuid().not_null())
                    .col(ColumnDef::new(Specialtie::Name).string_len(120).not_null())
                    .col(
                        ColumnDef::new(Specialtie::CreatedAt)
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
            .drop_table(Table::drop().table(Specialtie::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "specialties")]
pub enum Specialtie {
    Id,
    Table,
    CompanyId,
    Name,
    CreatedAt,
}
