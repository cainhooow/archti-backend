use sea_orm_migration::{prelude::*, schema::*, sea_orm::dynamic::Column};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .col(uuid(Client::Id).primary_key())
                    .col(ColumnDef::new(Client::CompanyId).uuid().not_null())
                    .col(ColumnDef::new(Client::CompanyMembershipId).uuid().null())
                    .col(ColumnDef::new(Client::FirstName).string_len(120).not_null())
                    .col(ColumnDef::new(Client::LastName).string_len(120).not_null())
                    .col(ColumnDef::new(Client::FullName).string_len(240).not_null())
                    .col(ColumnDef::new(Client::Email).string_len(160).null())
                    .col(ColumnDef::new(Client::Phone).string_len(32).null())
                    .col(ColumnDef::new(Client::Instagram).string_len(120).null())
                    .col(ColumnDef::new(Client::Document).string_len(32).null())
                    .col(ColumnDef::new(Client::Ocupation).string_len(120).null())
                    .col(ColumnDef::new(Client::City).string_len(120).null())
                    .col(ColumnDef::new(Client::Profile).string_len(60).null())
                    .col(ColumnDef::new(Client::StatusKey).string_len(60).not_null())
                    .col(ColumnDef::new(Client::Note).text().null())
                    .col(ColumnDef::new(Client::CustomerSinceAt).timestamp().null())
                    .col(ColumnDef::new(Client::LastContactAt).timestamp().null())
                    .col(ColumnDef::new(Client::ArchivedAt).timestamp().null())
                    .col(
                        ColumnDef::new(Client::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Client::UpdatedAt)
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
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "clients")]
pub enum Client {
    Id,
    Table,
    CompanyId,
    CompanyMembershipId,
    FirstName,
    LastName,
    FullName,
    Email,
    Phone,
    Instagram,
    Document,
    Ocupation,
    City,
    Profile,
    StatusKey,
    Note,
    CustomerSinceAt,
    LastContactAt,
    ArchivedAt,
    CreatedAt,
    UpdatedAt,
}
