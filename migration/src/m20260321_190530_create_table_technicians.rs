use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company,
    m20260321_144012_create_table_company_memberships::CompanyMembership,
    m20260321_150920_create_table_specialties::Specialtie,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(Technician::Table, Technician::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut company_membership_fk = ForeignKey::create()
            .from(Technician::Table, Technician::CompanyMembershipId)
            .to(CompanyMembership::Table, CompanyMembership::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut primary_specialty_fk = ForeignKey::create()
            .from(Technician::Table, Technician::PrimarySpecialtyId)
            .to(Specialtie::Table, Specialtie::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Technician::Table)
                    .col(big_integer(Technician::Id).primary_key())
                    .col(
                        ColumnDef::new(Technician::CompanyId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Technician::CompanyMembershipId)
                            .big_integer()
                            .unique_key()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Technician::PrimarySpecialtyId)
                            .big_integer()
                            .null(),
                    )
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
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut company_membership_fk)
                    .foreign_key(&mut primary_specialty_fk)
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
