use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderTimelineEvent::Table)
                    .col(uuid(OrderTimelineEvent::Id).primary_key())
                    .col(
                        ColumnDef::new(OrderTimelineEvent::OrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderTimelineEvent::Title)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrderTimelineEvent::Subtitle).text().null())
                    .col(
                        ColumnDef::new(OrderTimelineEvent::CreatedAt)
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
            .drop_table(Table::drop().table(OrderTimelineEvent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "order_timeline_events")]
pub enum OrderTimelineEvent {
    Id,
    Table,
    OrderId,
    Title,
    Subtitle,
    CreatedAt,
}
