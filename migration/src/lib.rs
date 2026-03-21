pub use sea_orm_migration::prelude::*;

mod m20260321_041852_create_table_companies;
mod m20260321_043556_create_table_users;
mod m20260321_050225_create_table_permissions;
mod m20260321_140601_create_table_plans;
mod m20260321_141334_create_table_plan_features;
mod m20260321_142529_create_table_company_addresses;
mod m20260321_144012_create_table_company_memberships;
mod m20260321_144722_create_table_roles;
mod m20260321_145219_create_table_plan_versions;
mod m20260321_150920_create_table_specialties;
mod m20260321_151319_create_table_certifications;
mod m20260321_151931_create_table_payment_methods;
mod m20260321_153334_create_table_service_catalog_items;
mod m20260321_154602_create_table_service_expense_presets;
mod m20260321_160001_create_table_service_order_status_setps;
mod m20260321_161049_create_table_stock_products;
mod m20260321_173415_create_table_resources;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
