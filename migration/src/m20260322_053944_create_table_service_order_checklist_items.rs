use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260322_040306_create_table_service_order_checklists::ServiceOrderChecklist;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut checklist_fk = ForeignKey::create()
            .from(
                ServiceOrderChecklistItem::Table,
                ServiceOrderChecklistItem::ChecklistId,
            )
            .to(ServiceOrderChecklist::Table, ServiceOrderChecklist::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderChecklistItem::Table)
                    .col(big_integer(ServiceOrderChecklistItem::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderChecklistItem::ChecklistId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistItem::Item)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistItem::Problem)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistItem::ImageNote)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistItem::SortOrder)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(&mut checklist_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ServiceOrderChecklistItem::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_checklist_items")]
pub enum ServiceOrderChecklistItem {
    Id,
    Table,
    ChecklistId,
    Item,
    Problem,
    ImageNote,
    SortOrder,
}
