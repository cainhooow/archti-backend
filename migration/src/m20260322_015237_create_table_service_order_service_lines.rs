use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_153334_create_table_service_catalog_items::ServiceCatalogItem,
    m20260322_000414_create_table_service_orders::ServiceOrder,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut service_order_fk = ForeignKey::create()
            .from(
                ServiceOrderServiceLine::Table,
                ServiceOrderServiceLine::ServiceOrderId,
            )
            .to(ServiceOrder::Table, ServiceOrder::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut service_catalog_item_fk = ForeignKey::create()
            .from(
                ServiceOrderServiceLine::Table,
                ServiceOrderServiceLine::ServiceCatalogItemId,
            )
            .to(ServiceCatalogItem::Table, ServiceCatalogItem::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderServiceLine::Table)
                    .col(uuid(ServiceOrderServiceLine::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::ServiceOrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::ServiceCatalogItemId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::ServiceCodeSnapshot).string_len(40),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::ServiceNameSnapshot)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::CategorySnapshot)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::UnitPriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceOrderServiceLine::Note).text().null())
                    .col(
                        ColumnDef::new(ServiceOrderServiceLine::SortOrder)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(&mut service_order_fk)
                    .foreign_key(&mut service_catalog_item_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ServiceOrderServiceLine::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_order_service_lines")]
pub enum ServiceOrderServiceLine {
    Id,
    Table,
    ServiceOrderId,
    ServiceCatalogItemId,
    ServiceCodeSnapshot,
    ServiceNameSnapshot,
    CategorySnapshot,
    Quantity,
    UnitPriceCents,
    Note,
    SortOrder,
}
