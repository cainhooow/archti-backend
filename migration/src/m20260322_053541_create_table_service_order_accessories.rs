use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260322_014211_create_table_service_order_equipment_snapshots::ServiceOrderEquipmentSnapshot;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut so_equipment_snapshot_fk = ForeignKey::create()
            .from(
                ServiceOrderAccessory::Table,
                ServiceOrderAccessory::ServiceOrderEquipmentSnapshotId,
            )
            .to(
                ServiceOrderEquipmentSnapshot::Table,
                ServiceOrderEquipmentSnapshot::Id,
            )
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderAccessory::Table)
                    .col(big_integer(ServiceOrderAccessory::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderAccessory::ServiceOrderEquipmentSnapshotId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderAccessory::Description)
                            .string_len(180)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceOrderAccessory::SortOrder).integer())
                    .foreign_key(&mut so_equipment_snapshot_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceOrderAccessory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_accessories")]
pub enum ServiceOrderAccessory {
    Id,
    Table,
    ServiceOrderEquipmentSnapshotId,
    Description,
    SortOrder,
}
