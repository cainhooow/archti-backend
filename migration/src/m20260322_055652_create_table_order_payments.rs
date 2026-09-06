use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_151931_create_table_payment_methods::PaymentMethod,
    m20260322_051337_create_table_orders::Order,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut order_fk = ForeignKey::create()
            .from(OrderPayment::Table, OrderPayment::OrderId)
            .to(Order::Table, Order::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut payment_method_fk = ForeignKey::create()
            .from(OrderPayment::Table, OrderPayment::PaymentMethodId)
            .to(PaymentMethod::Table, PaymentMethod::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(OrderPayment::Table)
                    .col(big_integer(OrderPayment::Id).primary_key())
                    .col(
                        ColumnDef::new(OrderPayment::OrderId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderPayment::PaymentMethodId)
                            .big_integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrderPayment::MethodSnapshot)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderPayment::AmountCents)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OrderPayment::ReceivedAt).timestamp().null())
                    .col(ColumnDef::new(OrderPayment::Note).text().null())
                    .foreign_key(&mut order_fk)
                    .foreign_key(&mut payment_method_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderPayment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "order_payments")]
pub enum OrderPayment {
    Id,
    Table,
    OrderId,
    PaymentMethodId,
    MethodSnapshot,
    AmountCents,
    ReceivedAt,
    Note,
}
