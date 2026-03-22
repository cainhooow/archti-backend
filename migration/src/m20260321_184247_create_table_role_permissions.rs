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
                    .col(uuid(RolePermission::RoleId).not_null())
                    .col(uuid(RolePermission::PermissionId).not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-role_permission")
                            .col(RolePermission::RoleId)
                            .col(RolePermission::PermissionId)
                            .primary(),
                    )
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
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await
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
