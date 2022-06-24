use entity::owner;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::{MigrationName, MigrationTrait};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000009_populate_pets"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        owner::ActiveModel {
            first_name: Set("George".to_owned()),
            last_name: Set("Franklin".to_owned()),
            address: Set("110 W. Liberty St.".to_owned()),
            city: Set("Madison".to_owned()),
            telephone: Set("6085551023".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;


        owner::ActiveModel {
            first_name: Set("Betty".to_owned()),
            last_name: Set("Davis".to_owned()),
            address: Set("638 Cardinal Ave.".to_owned()),
            city: Set("Sun Prairie".to_owned()),
            telephone: Set("6085551749".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }
}
