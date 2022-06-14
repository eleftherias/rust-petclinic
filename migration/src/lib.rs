pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_vets_table;
mod m20220101_000002_index_vets_table;
mod m20220101_000003_create_specialties_table;
mod m20220101_000004_index_specialties_table;
mod m20220101_000005_create_vet_specialties_table;
mod m20220101_000006_populate_vets;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_vets_table::Migration),
            Box::new(m20220101_000002_index_vets_table::Migration),
            Box::new(m20220101_000003_create_specialties_table::Migration),
            Box::new(m20220101_000004_index_specialties_table::Migration),
            Box::new(m20220101_000005_create_vet_specialties_table::Migration),
            Box::new(m20220101_000006_populate_vets::Migration),
        ]
    }
}
