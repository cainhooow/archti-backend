use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CompanySubscriptionEvent::Table)
                    .col(uuid(CompanySubscriptionEvent::Id).primary_key())
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::CompanySubscriptionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::CreatedByMembershipId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::EventType)
                            .string_len(60)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::FromPlanVersionId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::ToPlanVersionId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::FromStatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::ToStatusKey)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::Notes)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::EffectiveAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CompanySubscriptionEvent::CreatedAt)
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
                    .table(CompanySubscriptionEvent::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "company_subscription_events")]
pub enum CompanySubscriptionEvent {
    Id,
    Table,
    CompanySubscriptionId,
    CreatedByMembershipId,
    EventType,
    FromPlanVersionId,
    ToPlanVersionId,
    FromStatusKey,
    ToStatusKey,
    Notes,
    EffectiveAt,
    CreatedAt,
}
