use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderPaymentMethod::Table)
                    .col(uuid(ServiceOrderPaymentMethod::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderPaymentMethod::ServiceOrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderPaymentMethod::PaymentMethodId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderPaymentMethod::SortOrder)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceOrderPaymentMethod::Table).to_owned())
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
