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
mod m20260321_175712_create_table_plan_version_features;
mod m20260321_180536_create_table_company_subscriptions;
mod m20260321_182739_create_table_company_feature_overrides;
mod m20260321_184247_create_table_role_permissions;
mod m20260321_185438_create_table_membership_roles;
mod m20260321_190530_create_table_technicians;
mod m20260321_195712_create_table_clients;
mod m20260321_201053_create_table_inventory_items;
mod m20260321_204738_create_table_resource_machines;
mod m20260321_205617_create_table_company_subscription_seats;
mod m20260321_210221_create_table_company_subscription_events;
mod m20260321_211034_create_table_technician_specialties;
mod m20260321_211506_create_table_technician_certifications;
mod m20260321_211923_create_table_client_addresses;
mod m20260321_232129_create_table_client_equipments;
mod m20260321_233441_create_table_company_subscription_addons;
mod m20260321_234254_create_table_subscription_invoices;
mod m20260322_000414_create_table_service_orders;
mod m20260322_011150_create_table_quotes;
mod m20260322_014211_create_table_service_order_equipment_snapshots;
mod m20260322_015237_create_table_service_order_service_lines;
mod m20260322_034301_create_table_service_order_expense_lines;
mod m20260322_035016_create_table_service_order_payment_methods;
mod m20260322_035544_create_table_service_order_timeline_events;
mod m20260322_040306_create_table_service_order_checklists;
mod m20260322_040822_create_table_quote_items;
mod m20260322_051337_create_table_orders;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260321_041852_create_table_companies::Migration),
            Box::new(m20260321_043556_create_table_users::Migration),
            Box::new(m20260321_050225_create_table_permissions::Migration),
            Box::new(m20260321_140601_create_table_plans::Migration),
            Box::new(m20260321_141334_create_table_plan_features::Migration),
            Box::new(m20260321_142529_create_table_company_addresses::Migration),
            Box::new(m20260321_144012_create_table_company_memberships::Migration),
            Box::new(m20260321_144722_create_table_roles::Migration),
            Box::new(m20260321_145219_create_table_plan_versions::Migration),
            Box::new(m20260321_150920_create_table_specialties::Migration),
            Box::new(m20260321_151319_create_table_certifications::Migration),
            Box::new(m20260321_151931_create_table_payment_methods::Migration),
            Box::new(m20260321_153334_create_table_service_catalog_items::Migration),
            Box::new(m20260321_154602_create_table_service_expense_presets::Migration),
            Box::new(m20260321_160001_create_table_service_order_status_setps::Migration),
            Box::new(m20260321_161049_create_table_stock_products::Migration),
            Box::new(m20260321_173415_create_table_resources::Migration),
            Box::new(m20260321_175712_create_table_plan_version_features::Migration),
            Box::new(m20260321_180536_create_table_company_subscriptions::Migration),
            Box::new(m20260321_182739_create_table_company_feature_overrides::Migration),
            Box::new(m20260321_184247_create_table_role_permissions::Migration),
            Box::new(m20260321_185438_create_table_membership_roles::Migration),
            Box::new(m20260321_190530_create_table_technicians::Migration),
            Box::new(m20260321_195712_create_table_clients::Migration),
            Box::new(m20260321_201053_create_table_inventory_items::Migration),
            Box::new(m20260321_204738_create_table_resource_machines::Migration),
            Box::new(m20260321_205617_create_table_company_subscription_seats::Migration),
            Box::new(m20260321_210221_create_table_company_subscription_events::Migration),
            Box::new(m20260321_211034_create_table_technician_specialties::Migration),
            Box::new(m20260321_211506_create_table_technician_certifications::Migration),
            Box::new(m20260321_211923_create_table_client_addresses::Migration),
            Box::new(m20260321_232129_create_table_client_equipments::Migration),
            Box::new(m20260321_233441_create_table_company_subscription_addons::Migration),
            Box::new(m20260321_234254_create_table_subscription_invoices::Migration),
            Box::new(m20260322_000414_create_table_service_orders::Migration),
            Box::new(m20260322_011150_create_table_quotes::Migration),
            Box::new(m20260322_014211_create_table_service_order_equipment_snapshots::Migration),
            Box::new(m20260322_015237_create_table_service_order_service_lines::Migration),
            Box::new(m20260322_034301_create_table_service_order_expense_lines::Migration),
            Box::new(m20260322_035016_create_table_service_order_payment_methods::Migration),
            Box::new(m20260322_035544_create_table_service_order_timeline_events::Migration),
            Box::new(m20260322_040306_create_table_service_order_checklists::Migration),
            Box::new(m20260322_040822_create_table_quote_items::Migration),
            Box::new(m20260322_051337_create_table_orders::Migration),
        ]
    }
}
