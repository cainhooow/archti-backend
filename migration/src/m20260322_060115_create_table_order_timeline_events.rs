use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260322_051337_create_table_orders::Order;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut order_fk = ForeignKey::create()
            .from(OrderTimelineEvent::Table, OrderTimelineEvent::Id)
            .to(Order::Table, Order::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

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
                    .foreign_key(&mut order_fk)
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
