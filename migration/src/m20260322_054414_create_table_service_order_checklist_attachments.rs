use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260322_040306_create_table_service_order_checklists::ServiceOrderChecklist;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut checklist_fk = ForeignKey::create()
            .from(
                ServiceOrderChecklistAttachment::Table,
                ServiceOrderChecklistAttachment::Id,
            )
            .to(ServiceOrderChecklist::Table, ServiceOrderChecklist::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderChecklistAttachment::Table)
                    .col(uuid(ServiceOrderChecklistAttachment::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderChecklistAttachment::ChecklistId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistAttachment::FileName)
                            .string_len(180)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistAttachment::FileUrl)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistAttachment::ContentType)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistAttachment::Note)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderChecklistAttachment::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
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
                    .table(ServiceOrderChecklistAttachment::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_checklist_attachment")]
pub enum ServiceOrderChecklistAttachment {
    Id,
    Table,
    ChecklistId,
    FileName,
    FileUrl,
    ContentType,
    Note,
    CreatedAt,
}
