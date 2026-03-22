use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderChecklist::Table)
                    .col(uuid(ServiceOrderChecklist::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderChecklist::ServiceOrderId)
                            .uuid()
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
