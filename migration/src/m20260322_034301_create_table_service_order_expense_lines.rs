use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_154602_create_table_service_expense_presets::ServiceExpensePreset,
    m20260322_000414_create_table_service_orders::ServiceOrder,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut service_order_fk = ForeignKey::create()
            .from(
                ServiceOrderExpenseLine::Table,
                ServiceOrderExpenseLine::ServiceOrderId,
            )
            .to(ServiceOrder::Table, ServiceOrder::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut service_expense_preset_fk = ForeignKey::create()
            .from(
                ServiceOrderExpenseLine::Table,
                ServiceOrderExpenseLine::ServiceExpensePresetId,
            )
            .to(ServiceExpensePreset::Table, ServiceExpensePreset::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

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
                    .foreign_key(&mut service_order_fk)
                    .foreign_key(&mut service_expense_preset_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ServiceOrderExpenseLine::Table)
                    .to_owned(),
            )
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
