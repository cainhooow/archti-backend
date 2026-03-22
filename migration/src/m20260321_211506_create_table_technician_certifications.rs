use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TechnicianCertification::Table)
                    .col(uuid(TechnicianCertification::TechnicianId).not_null())
                    .col(uuid(TechnicianCertification::CertificationId).not_null())
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
