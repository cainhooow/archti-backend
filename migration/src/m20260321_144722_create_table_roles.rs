use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(Role::Table, Role::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .col(big_integer(Role::Id).primary_key())
                    .col(big_integer(Role::CompanyId).not_null())
                    .col(string_len(Role::Code, 60).not_null())
                    .col(string_len(Role::Name, 120).not_null())
                    .col(text(Role::Description).null())
                    .col(boolean(Role::IsSystemRole).default(true).not_null())
                    .col(
                        timestamp(Role::CreatedAt)
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
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "roles")]
pub enum Role {
    Id,
    Table,
    CompanyId,
    Code,
    Name,
    Description,
    IsSystemRole,
    CreatedAt,
}
