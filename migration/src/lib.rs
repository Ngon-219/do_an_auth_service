pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251028_000002_restructure_major_department;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251028_000002_restructure_major_department::Migration),
        ]
    }
}
