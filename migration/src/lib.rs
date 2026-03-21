pub use sea_orm_migration::prelude::*;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
mod m20260321_041852_create_table_companies;
mod m20260321_043556_create_table_users;
mod m20260321_050225_create_table_permissions;
mod m20260321_140601_create_table_plans;
mod m20260321_141334_create_table_plan_features;
