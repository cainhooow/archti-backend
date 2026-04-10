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
                    .table(User::Table)
                    .col(big_integer(User::Id).primary_key())
                    .col(string_len(User::Email, 160).not_null())
                    .col(string_len(User::PasswordHash, 255).not_null())
                    .col(string_len(User::FullName, 160).not_null())
                    .col(string_len(User::Phone, 32).null())
                    .col(string_len(User::StatusKey, 40).default("active").not_null())
                    .col(boolean(User::IsSuperAdmin).default(false).not_null())
                    .col(timestamp(User::LastLoginAt).null())
                    .col(timestamp(User::LastPasswordChangedAt).null())
                    .col(
                        timestamp(User::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(User::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "users")]
pub enum User {
    Id,
    Table,
    Email,
    PasswordHash,
    FullName,
    Phone,
    StatusKey,
    IsSuperAdmin,
    LastLoginAt,
    LastPasswordChangedAt,
    CreatedAt,
    UpdatedAt,
}
