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
                    .table(MembershipRole::Table)
                    .col(uuid(MembershipRole::MembershipId).primary_key().not_null())
                    .col(uuid(MembershipRole::RoleId).not_null())
                    .col(
                        ColumnDef::new(MembershipRole::CreatedAt)
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
#[sea_orm(table_name = "membership_roles")]
pub enum MembershipRole {
    Table,
    MembershipId,
    RoleId,
    CreatedAt,
}
