use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company, m20260321_043556_create_table_users::User,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

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
                    .col(
                        ColumnDef::new(CompanyMembership::CompanyId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::UserId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::MembershipType)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::StatusKey)
                            .string_len(40)
                            .default("active")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::DisplayName)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::InvitedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::AcceptedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::LastSeenAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::UpdatedAt)
                            .timestamp()
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
