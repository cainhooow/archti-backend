use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260321_043556_create_table_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut user_fk = ForeignKey::create()
            .from(PasswordResetToken::Table, PasswordResetToken::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(PasswordResetToken::Table)
                    .if_not_exists()
                    .col(uuid(PasswordResetToken::Id).primary_key())
                    .col(string(PasswordResetToken::Token).not_null())
                    .col(string(PasswordResetToken::UserId).not_null())
                    .foreign_key(&mut user_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordResetToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PasswordResetToken {
    Id,
    Table,
    Token,
    UserId,
}
