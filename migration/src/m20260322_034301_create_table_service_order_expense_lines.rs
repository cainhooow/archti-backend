use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderExpenseLine::Table)
                    .col(uuid(ServiceOrderExpenseLine::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::ServiceOrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::ServiceExpensePresetId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::Title)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::Category)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::AmountCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::Billable)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceOrderExpenseLine::Note).text().null())
                    .col(
                        ColumnDef::new(ServiceOrderExpenseLine::SortOrder)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("post").to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_expense_lines")]
pub enum ServiceOrderExpenseLine {
    Id,
    Table,
    ServiceOrderId,
    ServiceExpensePresetId,
    Title,
    Category,
    AmountCents,
    Billable,
    Note,
    SortOrder,
}
