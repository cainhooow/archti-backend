use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_151319_create_table_certifications::Certification,
    m20260321_190530_create_table_technicians::Technician,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut technician_fk = ForeignKey::create()
            .from(
                TechnicianCertification::Table,
                TechnicianCertification::TechnicianId,
            )
            .to(Technician::Table, Technician::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut certification_fk = ForeignKey::create()
            .from(
                TechnicianCertification::Table,
                TechnicianCertification::CertificationId,
            )
            .to(Certification::Table, Certification::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(TechnicianCertification::Table)
                    .col(big_integer(TechnicianCertification::TechnicianId).not_null())
                    .col(big_integer(TechnicianCertification::CertificationId).not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-technician_certifications")
                            .col(TechnicianCertification::TechnicianId)
                            .col(TechnicianCertification::CertificationId)
                            .primary(),
                    )
                    .col(
                        ColumnDef::new(TechnicianCertification::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut technician_fk)
                    .foreign_key(&mut certification_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(TechnicianCertification::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "technician_certifications")]
pub enum TechnicianCertification {
    Table,
    TechnicianId,
    CertificationId,
    CreatedAt,
}
