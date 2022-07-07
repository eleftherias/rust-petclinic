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

        pet_type::ActiveModel {
            name: Set("lizard".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet_type::ActiveModel {
            name: Set("snake".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet_type::ActiveModel {
            name: Set("bird".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet_type::ActiveModel {
            name: Set("hamster".to_owned()),
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

        pet::ActiveModel {
            name: Set("Basil".to_owned()),
            birth_date: Set(Date::parse_from_str("2012-08-06", "%Y-%m-%d").unwrap()),
            type_id: Set(6),
            owner_id: Set(Some(2)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Rosy".to_owned()),
            birth_date: Set(Date::parse_from_str("2011-04-17", "%Y-%m-%d").unwrap()),
            type_id: Set(2),
            owner_id: Set(Some(3)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Jewel".to_owned()),
            birth_date: Set(Date::parse_from_str("2010-03-07", "%Y-%m-%d").unwrap()),
            type_id: Set(2),
            owner_id: Set(Some(3)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Iggy".to_owned()),
            birth_date: Set(Date::parse_from_str("2010-11-30", "%Y-%m-%d").unwrap()),
            type_id: Set(3),
            owner_id: Set(Some(4)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("George".to_owned()),
            birth_date: Set(Date::parse_from_str("2010-01-20", "%Y-%m-%d").unwrap()),
            type_id: Set(4),
            owner_id: Set(Some(5)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Samantha".to_owned()),
            birth_date: Set(Date::parse_from_str("2012-09-04", "%Y-%m-%d").unwrap()),
            type_id: Set(1),
            owner_id: Set(Some(6)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Max".to_owned()),
            birth_date: Set(Date::parse_from_str("2012-09-04", "%Y-%m-%d").unwrap()),
            type_id: Set(1),
            owner_id: Set(Some(6)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Lucky".to_owned()),
            birth_date: Set(Date::parse_from_str("2011-08-06", "%Y-%m-%d").unwrap()),
            type_id: Set(5),
            owner_id: Set(Some(7)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Mulligan".to_owned()),
            birth_date: Set(Date::parse_from_str("2007-02-24", "%Y-%m-%d").unwrap()),
            type_id: Set(2),
            owner_id: Set(Some(8)),
            ..Default::default()
        }
        .insert(db)
        .await?;
        pet::ActiveModel {
            name: Set("Freddy".to_owned()),
            birth_date: Set(Date::parse_from_str("2010-03-09", "%Y-%m-%d").unwrap()),
            type_id: Set(5),
            owner_id: Set(Some(9)),
            ..Default::default()
        }
        .insert(db)
        .await?;
        pet::ActiveModel {
            name: Set("Lucky".to_owned()),
            birth_date: Set(Date::parse_from_str("2010-06-24", "%Y-%m-%d").unwrap()),
            type_id: Set(2),
            owner_id: Set(Some(10)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        pet::ActiveModel {
            name: Set("Sly".to_owned()),
            birth_date: Set(Date::parse_from_str("2012-06-08", "%Y-%m-%d").unwrap()),
            type_id: Set(1),
            owner_id: Set(Some(10)),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }
}
