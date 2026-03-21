use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentMethod::Table)
                    .col(uuid(PaymentMethod::Id).primary_key())
                    .col(ColumnDef::new(PaymentMethod::CompanyId).uuid().not_null())
                    .col(
                        ColumnDef::new(PaymentMethod::Title)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PaymentMethod::Helper).string_len(180).null())
                    .col(
                        ColumnDef::new(PaymentMethod::FeeHint)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethod::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethod::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table("post").to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "payment_methods")]
pub enum PaymentMethod {
    Id,
    Table,
    CompanyId,
    Title,
    Helper,
    FeeHint,
    IsActive,
    CreatedAt,
}
