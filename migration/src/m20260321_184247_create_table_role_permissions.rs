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
                    .table(RolePermission::Table)
                    .col(uuid(RolePermission::RoleId).primary_key().not_null())
                    .col(uuid(RolePermission::PermissionId).not_null())
                    .col(
                        ColumnDef::new(RolePermission::CreatedAt)
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
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "role_permissions")]
pub enum RolePermission {
    Table,
    RoleId,
    PermissionId,
    CreatedAt,
}
