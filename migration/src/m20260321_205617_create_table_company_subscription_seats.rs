use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_144012_create_table_company_memberships::CompanyMembership,
    m20260321_180536_create_table_company_subscriptions::CompanySubscription,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_subscription_fk = ForeignKey::create()
            .from(
                CompanySubscriptionSeat::Table,
                CompanySubscriptionSeat::CompanySubscriptionId,
            )
            .to(CompanySubscription::Table, CompanySubscription::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut company_membership_fk = ForeignKey::create()
            .from(
                CompanySubscriptionSeat::Table,
                CompanySubscriptionSeat::MembershipId,
            )
            .to(CompanyMembership::Table, CompanyMembership::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanySubscriptionSeat::Table)
                    .col(big_integer(CompanySubscriptionSeat::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::CompanySubscriptionId)
                            .big_integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionSeat::MembershipId)
                            .big_integer()
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
                    .foreign_key(&mut company_subscription_fk)
                    .foreign_key(&mut company_membership_fk)
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
