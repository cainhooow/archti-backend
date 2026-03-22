use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260321_041852_create_table_companies::Company,
    m20260321_160001_create_table_service_order_status_setps::ServiceOrderStatusStep,
    m20260321_190530_create_table_technicians::Technician,
    m20260321_195712_create_table_clients::Client,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut company_fk = ForeignKey::create()
            .from(ServiceOrder::Table, ServiceOrder::CompanyId)
            .to(Company::Table, Company::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut client_fk = ForeignKey::create()
            .from(ServiceOrder::Table, ServiceOrder::ClientId)
            .to(Client::Table, Client::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut technician_fk = ForeignKey::create()
            .from(ServiceOrder::Table, ServiceOrder::TechnicianId)
            .to(Technician::Table, Technician::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let mut order_status_step_fk = ForeignKey::create()
            .from(ServiceOrder::Table, ServiceOrder::CurrentStatusStepId)
            .to(ServiceOrderStatusStep::Table, ServiceOrderStatusStep::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrder::Table)
                    .col(uuid(ServiceOrder::Id).primary_key())
                    .col(ColumnDef::new(ServiceOrder::CompanyId).uuid().not_null())
                    .col(
                        ColumnDef::new(ServiceOrder::ServiceOrderNumber)
                            .string_len(40)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceOrder::ClientId).uuid().null())
                    .col(ColumnDef::new(ServiceOrder::TechnicianId).uuid().null())
                    .col(
                        ColumnDef::new(ServiceOrder::CurrentStatusStepId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::PriorityKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::CustomerNameSnapshot)
                            .string_len(140)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::CustomerEmailSnapshot)
                            .string_len(160)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::CustomerPhoneSnapshot)
                            .string_len(32)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::ProblemReport)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceOrder::IntakeNote).text().null())
                    .col(ColumnDef::new(ServiceOrder::Diagnosis).text().null())
                    .col(ColumnDef::new(ServiceOrder::ServicePlan).text().null())
                    .col(ColumnDef::new(ServiceOrder::CustomerContext).text().null())
                    .col(
                        ColumnDef::new(ServiceOrder::CommercialContext)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(ServiceOrder::DefectNote).text().null())
                    .col(ColumnDef::new(ServiceOrder::SolutionNote).text().null())
                    .col(ColumnDef::new(ServiceOrder::AdditionalInfo).text().null())
                    .col(
                        ColumnDef::new(ServiceOrder::DiscountMode)
                            .string_len(40)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::DiscountValueCents)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::DiscountValuePercent)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::SubtotalServicesCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::SubtotalExpensesCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::TotalCents)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::OpenedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::PromisedWindowAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceOrder::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(&mut company_fk)
                    .foreign_key(&mut client_fk)
                    .foreign_key(&mut technician_fk)
                    .foreign_key(&mut order_status_step_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceOrder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "service_orders")]
pub enum ServiceOrder {
    Id,
    Table,
    CompanyId,
    ServiceOrderNumber,
    ClientId,
    TechnicianId,
    CurrentStatusStepId,
    PriorityKey,
    CustomerNameSnapshot,
    CustomerEmailSnapshot,
    CustomerPhoneSnapshot,
    ProblemReport,
    IntakeNote,
    Diagnosis,
    ServicePlan,
    CustomerContext,
    CommercialContext,
    DefectNote,
    SolutionNote,
    AdditionalInfo,
    DiscountMode,
    DiscountValueCents,
    DiscountValuePercent,
    SubtotalServicesCents,
    SubtotalExpensesCents,
    TotalCents,
    OpenedAt,
    PromisedWindowAt,
    CreatedAt,
    UpdatedAt,
}
