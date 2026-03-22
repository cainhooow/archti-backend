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
                    .col(uuid(User::Id).primary_key())
                    .col(
                        ColumnDef::new(User::Email)
                            .unique_key()
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::PasswordHash)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::FullName).string_len(160).not_null())
                    .col(ColumnDef::new(User::Phone).string_len(32).null())
                    .col(
                        ColumnDef::new(User::StatusKey)
                            .string_len(40)
                            .default("active")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::IsSuperAdmin)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::LastLoginAt).timestamp().null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
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
    CreatedAt,
    UpdatedAt,
}
