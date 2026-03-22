use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderEquipmentSnapshot::Table)
                    .col(uuid(ServiceOrderEquipmentSnapshot::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::ServiceOrderId)
                            .uuid()
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
                        ColumnDef::new(ServiceOrderEquipmentSnapshot::CreatedAt)
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
