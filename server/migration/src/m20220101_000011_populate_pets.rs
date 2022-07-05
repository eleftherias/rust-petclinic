use entity::{pet, pet_type};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::prelude::Date;
use sea_orm_migration::sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::{MigrationName, MigrationTrait};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000011_populate_pets"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        pet_type::ActiveModel {
            name: Set("cat".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet_type::ActiveModel {
            name: Set("dog".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Leo".to_owned()),
            birth_date: Set(Date::parse_from_str("2000-09-07", "%Y-%m-%d").unwrap()),
            type_id: Set(1),
            owner_id: Set(Some(1)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }
}
