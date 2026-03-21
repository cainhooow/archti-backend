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
                    .table(Company::Table)
                    .col(uuid(Company::Id).primary_key())
                    .col(
                        ColumnDef::new(Company::LegalName)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::TradeName)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::ServiceType)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Company::Document).string_len(32).not_null())
                    .col(
                        ColumnDef::new(Company::ContactName)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Company::PrimaryPhone).string_len(32).null())
                    .col(
                        ColumnDef::new(Company::LicensePlan)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::LicenseStatus)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::LicenseDaysRemaining)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::OperationalBase)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Company::Notes).text().null())
                    .col(
                        ColumnDef::new(Company::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Company::UpdatedAt)
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
#[sea_orm(table_name = "companies")]
pub enum Company {
    Id,
    Table,
    LegalName,
    TradeName,
    ServiceType,
    Document,
    ContactName,
    PrimaryPhone,
    SecondaryPhone,
    LicensePlan,
    LicenseStatus,
    LicenseDaysRemaining,
    OperationalBase,
    Notes,
    CreatedAt,
    UpdatedAt,
}
