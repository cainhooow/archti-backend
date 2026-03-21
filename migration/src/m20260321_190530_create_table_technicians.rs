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
                    .table(Technician::Table)
                    .col(uuid(Technician::Id).primary_key())
                    .col(ColumnDef::new(Technician::CompanyId).uuid().not_null())
                    .col(
                        ColumnDef::new(Technician::CompanyMembershipId)
                            .uuid()
                            .null(),
                    )
                    .col(ColumnDef::new(Technician::PrimarySpecialtyId).uuid().null())
                    .col(ColumnDef::new(Technician::Name).string_len(120).not_null())
                    .col(ColumnDef::new(Technician::Role).string_len(120).not_null())
                    .col(
                        ColumnDef::new(Technician::StatusKey)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Technician::Phone).string_len(32).not_null())
                    .col(ColumnDef::new(Technician::Email).string_len(160).null())
                    .col(
                        ColumnDef::new(Technician::CityLabel)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Technician::ShiftLabel)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Technician::Note).text().null())
                    .col(
                        ColumnDef::new(Technician::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Technician::UpdatedAt)
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
            .drop_table(Table::drop().table(Technician::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "technicians")]
pub enum Technician {
    Id,
    Table,
    CompanyId,
    CompanyMembershipId,
    PrimarySpecialtyId,
    Name,
    Role,
    StatusKey,
    Phone,
    Email,
    CityLabel,
    ShiftLabel,
    Note,
    CreatedAt,
    UpdatedAt,
}
