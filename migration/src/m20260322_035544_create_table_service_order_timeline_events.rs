use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderTimelineEvent::Table)
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::ServiceOrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::StatusStepId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::Title)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::Subtitle)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::CreatedAt)
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
                    .table(ServiceOrderTimelineEvent::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_timeline_events")]
pub enum ServiceOrderTimelineEvent {
    Id,
    Table,
    ServiceOrderId,
    StatusStepId,
    Title,
    Subtitle,
    CreatedAt,
}
