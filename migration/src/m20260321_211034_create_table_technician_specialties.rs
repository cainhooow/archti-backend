use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TechnicianSpeciality::Table)
                    .col(uuid(TechnicianSpeciality::TechnicianId).not_null())
                    .col(uuid(TechnicianSpeciality::SpecialtyId).not_null())
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
