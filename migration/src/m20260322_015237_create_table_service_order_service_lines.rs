use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
