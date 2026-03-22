use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_195712_create_table_clients::Client,
    m20260321_211923_create_table_client_addresses::ClientAddress,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut client_fk = ForeignKey::create()
            .from(ClientAddress::Table, ClientAddress::ClientId)
            .to(Client::Table, Client::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ClientEquipment::Table)
                    .col(uuid(ClientEquipment::Id).primary_key())
                    .col(ColumnDef::new(ClientEquipment::ClientId).uuid().not_null())
                    .col(
                        ColumnDef::new(ClientEquipment::Name)
                            .string_len(160)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientEquipment::Category)
                            .string_len(80)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientEquipment::SerialNumber)
                            .string_len(120)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientEquipment::StatusKey)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ClientEquipment::Note).text().not_null())
                    .col(
                        ColumnDef::new(ClientEquipment::LastServiceAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ClientEquipment::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ClientEquipment::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut client_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClientEquipment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "client_equipments")]
pub enum ClientEquipment {
    Id,
    Table,
    ClientId,
    Name,
    Category,
    SerialNumber,
    StatusKey,
    Note,
    LastServiceAt,
    CreatedAt,
    UpdatedAt,
}
