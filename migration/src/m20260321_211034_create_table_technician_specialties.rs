use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_150920_create_table_specialties::Specialtie,
    m20260321_190530_create_table_technicians::Technician,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut technician_fk = ForeignKey::create()
            .from(
                TechnicianSpeciality::Table,
                TechnicianSpeciality::TechnicianId,
            )
            .to(Technician::Table, Technician::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut specialty_fk = ForeignKey::create()
            .from(
                TechnicianSpeciality::Table,
                TechnicianSpeciality::SpecialtyId,
            )
            .to(Specialtie::Table, Specialtie::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(TechnicianSpeciality::Table)
                    .col(big_integer(TechnicianSpeciality::TechnicianId).not_null())
                    .col(big_integer(TechnicianSpeciality::SpecialtyId).not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-technician_specialties")
                            .col(TechnicianSpeciality::TechnicianId)
                            .col(TechnicianSpeciality::SpecialtyId)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(TechnicianSpeciality::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut technician_fk)
                    .foreign_key(&mut specialty_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TechnicianSpeciality::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "technician_specialties")]
pub enum TechnicianSpeciality {
    Table,
    TechnicianId,
    SpecialtyId,
    CreatedAt,
}
