use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_144012_create_table_company_memberships::CompanyMembership,
    m20260321_144722_create_table_roles::Role,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut membership_fk = ForeignKey::create()
            .from(MembershipRole::Table, MembershipRole::MembershipId)
            .to(CompanyMembership::Table, CompanyMembership::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut role_fk = ForeignKey::create()
            .from(MembershipRole::Table, MembershipRole::RoleId)
            .to(Role::Table, Role::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(MembershipRole::Table)
                    .col(big_integer(MembershipRole::MembershipId).not_null())
                    .col(big_integer(MembershipRole::RoleId).not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-membership_roles")
                            .col(MembershipRole::MembershipId)
                            .col(MembershipRole::RoleId)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(MembershipRole::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut membership_fk)
                    .foreign_key(&mut role_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MembershipRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "membership_roles")]
pub enum MembershipRole {
    Table,
    MembershipId,
    RoleId,
    CreatedAt,
}
