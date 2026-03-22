use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CompanySubscriptionSeat::Table)
                    .col(uuid(CompanySubscriptionSeat::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::CompanySubscriptionId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::MembershipId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::SeatKind)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::StatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::BilledFrom)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::BilledUntil)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::CreatedAt)
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
                    .table(CompanySubscriptionSeat::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "comapany_subscription_seats")]
pub enum CompanySubscriptionSeat {
    Id,
    Table,
    CompanySubscriptionId,
    MembershipId,
    SeatKind,
    StatusKey,
    BilledFrom,
    BilledUntil,
    CreatedAt,
}
