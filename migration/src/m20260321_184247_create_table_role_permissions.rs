use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_050225_create_table_permissions::Permission,
    m20260321_144722_create_table_roles::Role,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut role_fk = ForeignKey::create()
            .from(RolePermission::Table, RolePermission::RoleId)
            .to(Role::Table, Role::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut permission_fk = ForeignKey::create()
            .from(RolePermission::Table, RolePermission::PermissionId)
            .to(Permission::Table, Permission::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .col(big_integer(RolePermission::RoleId).not_null())
                    .col(big_integer(RolePermission::PermissionId).not_null())
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
                    .foreign_key(&mut role_fk)
                    .foreign_key(&mut permission_fk)
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
