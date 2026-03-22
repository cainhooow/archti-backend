use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderPayment::Table)
                    .col(ColumnDef::new(OrderPayment::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderPayment::PaymentMethodId).uuid().null())
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
