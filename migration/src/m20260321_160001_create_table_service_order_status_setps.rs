use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(
                ServiceOrderStatusStep::Table,
                ServiceOrderStatusStep::CompanyId,
            )
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderStatusStep::Table)
                    .col(uuid(ServiceOrderStatusStep::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::CompanyId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::StatusKey)
                            .string_len(60)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::Title)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::Description)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(ServiceOrderStatusStep::Helper).text().null())
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::SortOrder)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::IsActive)
                            .boolean()
                            .default(Expr::value(true))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderStatusStep::CreatedAt)
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
            .drop_table(
                Table::drop()
                    .table(ServiceOrderStatusStep::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "sercice_order_status_steps")]
pub enum ServiceOrderStatusStep {
    Id,
    Table,
    CompanyId,
    StatusKey,
    Title,
    Description,
    Helper,
    SortOrder,
    IsActive,
    CreatedAt,
}
