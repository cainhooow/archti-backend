use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderChecklistItem::Table)
                    .col(uuid(ServiceOrderChecklistItem::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderChecklistItem::ChecklistId)
                            .uuid()
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
