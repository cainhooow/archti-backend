use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(CompanyMembership::Table)
                    .col(uuid(CompanyMembership::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanyMembership::CompanyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CompanyMembership::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(CompanyMembership::MembershipType)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanyMembership::StatusKey)
                            .string_len(40)
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
