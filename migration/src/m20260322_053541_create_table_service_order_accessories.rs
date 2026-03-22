use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("post")
                    .col(uuid(ServiceOrderAccessory::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderAccessory::ServiceOrderEquipmentSnapshotId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderAccessory::Description)
                            .string_len(180)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceOrderAccessory::SortOrder).integer())
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
