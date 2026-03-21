use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceCatalogItem::Table)
                    .col(uuid(ServiceCatalogItem::Id).primary_key())
                    .col(
                        ColumnDef::new(ServiceCatalogItem::CompanyId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::Code)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::Name)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::Category)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::BasePriceCents)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::TurnaroundLabel)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::BillingType)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::MaterialsHint)
                            .string_len(180)
                            .null(),
                    )
                    .col(ColumnDef::new(ServiceCatalogItem::Note).text().null())
                    .col(
                        ColumnDef::new(ServiceCatalogItem::IsActive)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceCatalogItem::UpdatedAt)
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
#[sea_orm(table_name = "service_catalog_items")]
pub enum ServiceCatalogItem {
    Id,
    Table,
    CompanyId,
    Code,
    Name,
    Category,
    BasePriceCents,
    TurnaroundLabel,
    BillingType,
    MaterialsHint,
    Note,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
