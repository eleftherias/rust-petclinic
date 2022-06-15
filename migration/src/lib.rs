pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_vets_table;
mod m20220101_000002_index_vets_table;
mod m20220101_000003_create_specialties_table;
mod m20220101_000004_index_specialties_table;
mod m20220101_000005_create_vet_specialties_table;
mod m20220101_000006_populate_vets;
mod m20220101_000007_create_pet_types_table;
mod m20220101_000008_create_owners_table;
mod m20220101_000009_create_pets_table;
mod m20220101_000010_populate_owners;
mod m20220101_000011_populate_pets;

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
            Box::new(m20220101_000007_create_pet_types_table::Migration),
            Box::new(m20220101_000008_create_owners_table::Migration),
            Box::new(m20220101_000009_create_pets_table::Migration),
            Box::new(m20220101_000010_populate_owners::Migration),
            Box::new(m20220101_000011_populate_pets::Migration),
        ]
    }
}
