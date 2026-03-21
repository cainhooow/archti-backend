use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table("post").to_owned())
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
