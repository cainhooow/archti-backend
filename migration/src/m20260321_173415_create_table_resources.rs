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
                    .table(Resource::Table)
                    .col(uuid(Resource::Id).primary_key())
                    .col(ColumnDef::new(Resource::CompanyId).uuid().not_null())
                    .col(ColumnDef::new(Resource::Code).string_len(60).not_null())
                    .col(ColumnDef::new(Resource::Name).string_len(160).not_null())
                    .col(
                        ColumnDef::new(Resource::ResourceType)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Resource::ControlType)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Resource::Unit).string_len(40).not_null())
                    .col(ColumnDef::new(Resource::Quantity).integer().not_null())
                    .col(ColumnDef::new(Resource::Minimum).integer().not_null())
                    .col(
                        ColumnDef::new(Resource::ItemCondition)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Resource::Status).string_len(60).not_null())
                    .col(
                        ColumnDef::new(Resource::Location)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Resource::Supplier)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Resource::Brand).string_len(120).not_null())
                    .col(ColumnDef::new(Resource::Application).text().not_null())
                    .col(ColumnDef::new(Resource::AssetTag).string_len(60).null())
                    .col(
                        ColumnDef::new(Resource::SerialNumber)
                            .string_len(120)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Resource::AverageCostCents)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Resource::LastEntryAt).date().null())
                    .col(ColumnDef::new(Resource::Notes).text().null())
                    .col(
                        ColumnDef::new(Resource::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Resource::UpdatedAt)
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
            .drop_table(Table::drop().table(Resource::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "resources")]
pub enum Resource {
    Id,
    Table,
    CompanyId,
    Code,
    Name,
    ResourceType,
    ControlType,
    Unit,
    Quantity,
    Minimum,
    ItemCondition,
    Status,
    Location,
    Supplier,
    Brand,
    Application,
    AssetTag,
    SerialNumber,
    AverageCostCents,
    LastEntryAt,
    Notes,
    CreatedAt,
    UpdatedAt,
}
