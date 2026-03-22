use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_041852_create_table_companies::Company;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(ServiceExpensePreset::Table, ServiceExpensePreset::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceExpensePreset::Table)
                    .col(uuid(ServiceExpensePreset::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceExpensePreset::CompanyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceExpensePreset::Title)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceExpensePreset::Category)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceExpensePreset::AmountCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceExpensePreset::Billable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceExpensePreset::Note).text().null())
                    .col(
                        ColumnDef::new(ServiceExpensePreset::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceExpensePreset::CreatedAt)
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
            .drop_table(Table::drop().table(ServiceExpensePreset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_expense_presets")]
pub enum ServiceExpensePreset {
    Id,
    Table,
    CompanyId,
    Title,
    Category,
    AmountCents,
    Billable,
    Note,
    IsActive,
    CreatedAt,
}
