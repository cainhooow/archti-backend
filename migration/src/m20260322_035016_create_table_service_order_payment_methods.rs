use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_151931_create_table_payment_methods::PaymentMethod,
    m20260322_000414_create_table_service_orders::ServiceOrder,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut service_order_fk = ForeignKey::create()
            .from(
                ServiceOrderPaymentMethod::Table,
                ServiceOrderPaymentMethod::ServiceOrderId,
            )
            .to(ServiceOrder::Table, ServiceOrder::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut payment_method_fk = ForeignKey::create()
            .from(
                ServiceOrderPaymentMethod::Table,
                ServiceOrderPaymentMethod::PaymentMethodId,
            )
            .to(PaymentMethod::Table, PaymentMethod::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderPaymentMethod::Table)
                    .col(big_integer(ServiceOrderPaymentMethod::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderPaymentMethod::ServiceOrderId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderPaymentMethod::PaymentMethodId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderPaymentMethod::SortOrder)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(&mut service_order_fk)
                    .foreign_key(&mut payment_method_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ServiceOrderPaymentMethod::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_payment_methods")]
pub enum ServiceOrderPaymentMethod {
    Id,
    Table,
    ServiceOrderId,
    PaymentMethodId,
    SortOrder,
}
