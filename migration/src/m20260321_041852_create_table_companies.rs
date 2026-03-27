use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Company::Table)
                    .col(uuid(Company::Id).primary_key())
                    .col(string_len(Company::LegalName, 160).not_null())
                    .col(string_len(Company::TradeName, 160).not_null())
                    .col(string_len(Company::ServiceType, 120).not_null())
                    .col(string_len(Company::Document, 32).unique_key().not_null())
                    .col(string_len(Company::ContactName, 120).not_null())
                    .col(string_len(Company::PrimaryPhone, 32).null())
                    .col(string_len(Company::LicensePlan, 80).not_null())
                    .col(string_len(Company::LicenseStatus, 80).not_null())
                    .col(integer(Company::LicenseDaysRemaining).not_null())
                    .col(string_len(Company::OperationalBase, 120).not_null())
                    .col(text(Company::Notes).null())
                    .col(
                        timestamp(Company::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Company::UpdatedAt)
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
            .drop_table(Table::drop().table(Company::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
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
