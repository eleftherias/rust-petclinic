use entity::{specialty, vet, vet_specialty};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::{MigrationName, MigrationTrait};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000006_populate_vets"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        vet::ActiveModel {
            first_name: Set("James".to_owned()),
            last_name: Set("Carter".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        vet::ActiveModel {
            first_name: Set("Helen".to_owned()),
            last_name: Set("Leary".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        specialty::ActiveModel {
            name: Set("radiology".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        vet_specialty::ActiveModel {
            vet_id: Set(2),
            specialty_id: Set(1),
        }
        .insert(db)
        .await?;

        Ok(())
    }
}
