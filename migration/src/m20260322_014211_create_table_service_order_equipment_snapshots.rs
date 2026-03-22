use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_232129_create_table_client_equipments::ClientEquipment,
    m20260322_000414_create_table_service_orders::ServiceOrder,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut service_order_fk = ForeignKey::create()
            .from(
                ServiceOrderEquipmentSnapshot::Table,
                ServiceOrderEquipmentSnapshot::ServiceOrderId,
            )
            .to(ServiceOrder::Table, ServiceOrder::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut client_equipment_fk = ForeignKey::create()
            .from(
                ServiceOrderEquipmentSnapshot::Table,
                ServiceOrderEquipmentSnapshot::ClientEquipmentId,
            )
            .to(ClientEquipment::Table, ClientEquipment::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderEquipmentSnapshot::Table)
                    .col(uuid(ServiceOrderEquipmentSnapshot::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::ServiceOrderId)
                            .uuid()
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::ClientEquipmentId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::EquipmentName)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::Category)
                            .string_len(80)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::SerialNumber)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::IntakeCondition)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::UnlockCode)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::Notes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut service_order_fk)
                    .foreign_key(&mut client_equipment_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ServiceOrderEquipmentSnapshot::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_equipment_snapshots")]
pub enum ServiceOrderEquipmentSnapshot {
    Id,
    Table,
    ServiceOrderId,
    ClientEquipmentId,
    EquipmentName,
    Category,
    SerialNumber,
    IntakeCondition,
    UnlockCode,
    Notes,
    CreatedAt,
}
