use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
