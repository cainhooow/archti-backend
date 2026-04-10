use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260322_000414_create_table_service_orders::ServiceOrder;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut service_order_fk = ForeignKey::create()
            .from(
                ServiceOrderChecklist::Table,
                ServiceOrderChecklist::ServiceOrderId,
            )
            .to(ServiceOrder::Table, ServiceOrder::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderChecklist::Table)
                    .col(big_integer(ServiceOrderChecklist::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderChecklist::ServiceOrderId)
                            .big_integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklist::InspectionNotes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklist::SafetyNotes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklist::AttachmentNotes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklist::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut service_order_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceOrderChecklist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_checklists")]
pub enum ServiceOrderChecklist {
    Id,
    Table,
    ServiceOrderId,
    InspectionNotes,
    SafetyNotes,
    AttachmentNotes,
    UpdatedAt,
}
