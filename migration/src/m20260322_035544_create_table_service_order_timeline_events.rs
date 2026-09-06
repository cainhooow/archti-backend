use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_160001_create_table_service_order_status_setps::ServiceOrderStatusStep,
    m20260322_000414_create_table_service_orders::ServiceOrder,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut service_order_fk = ForeignKey::create()
            .from(
                ServiceOrderTimelineEvent::Table,
                ServiceOrderTimelineEvent::ServiceOrderId,
            )
            .to(ServiceOrder::Table, ServiceOrder::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut status_step_fk = ForeignKey::create()
            .from(
                ServiceOrderTimelineEvent::Table,
                ServiceOrderTimelineEvent::StatusStepId,
            )
            .to(ServiceOrderStatusStep::Table, ServiceOrderStatusStep::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderTimelineEvent::Table)
                    .col(big_integer(ServiceOrderTimelineEvent::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::ServiceOrderId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderTimelineEvent::StatusStepId)
                            .big_integer()
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
                    .foreign_key(&mut service_order_fk)
                    .foreign_key(&mut status_step_fk)
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
