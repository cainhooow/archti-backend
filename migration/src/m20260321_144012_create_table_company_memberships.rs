use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company, m20260321_043556_create_table_users::User,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(CompanyMembership::Table, CompanyMembership::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();
        let mut user_fk = ForeignKey::create()
            .from(CompanyMembership::Table, CompanyMembership::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(CompanyMembership::Table)
                    .col(uuid(CompanyMembership::Id).primary_key())
                    .col(uuid(CompanyMembership::CompanyId).not_null())
                    .col(uuid(CompanyMembership::UserId).not_null())
                    .col(string_len(CompanyMembership::MembershipType, 40).not_null())
                    .col(
                        string_len(CompanyMembership::StatusKey, 40)
                            .default("active")
                            .not_null(),
                    )
                    .col(string(CompanyMembership::DisplayName).null())
                    .col(timestamp(CompanyMembership::InvitedAt).null())
                    .col(timestamp(CompanyMembership::AcceptedAt).null())
                    .col(timestamp(CompanyMembership::LastSeenAt).null())
                    .col(
                        timestamp(CompanyMembership::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(CompanyMembership::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut user_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CompanyMembership::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_memberships")]
pub enum CompanyMembership {
    Id,
    Table,
    CompanyId,
    UserId,
    MembershipType,
    StatusKey,
    DisplayName,
    InvitedAt,
    AcceptedAt,
    LastSeenAt,
    CreatedAt,
    UpdatedAt,
}
