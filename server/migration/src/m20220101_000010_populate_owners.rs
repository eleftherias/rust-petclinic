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


        owner::ActiveModel {
            first_name: Set("Eduardo".to_owned()), 
            last_name: Set("Rodriquez".to_owned()), 
            address: Set("2693 Commerce St.".to_owned()),
            city: Set("McFarland".to_owned()),
            telephone: Set("6085558763".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("Harold".to_owned()), 
            last_name: Set("Davis".to_owned()), 
            address: Set("563 Friendly St.".to_owned()),
            city: Set("Windsor".to_owned()),
            telephone: Set("6085553198".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("Peter".to_owned()), 
            last_name: Set("McTavish".to_owned()), 
            address: Set("2387 S. Fair Way".to_owned()),
            city: Set("Madison".to_owned()),
            telephone: Set("6085552765".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("Jean".to_owned()), 
            last_name: Set("Coleman".to_owned()), 
            address: Set("105 N. Lake St.".to_owned()),
            city: Set("Monona".to_owned()),
             telephone: Set("6085552654".to_owned()),
             ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("Jeff".to_owned()), 
            last_name: Set("Black".to_owned()), 
            address: Set("1450 Oak Blvd.".to_owned()),
            city: Set("Monona".to_owned()),
            telephone: Set("6085555387".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("Maria".to_owned()), 
            last_name: Set("Escobito".to_owned()), 
            address: Set("345 Maple St.".to_owned()),
            city: Set("Madison".to_owned()),
            telephone: Set("6085557683".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("David".to_owned()), 
            last_name: Set("Schroeder".to_owned()), 
            address: Set("2749 Blackhawk Trail".to_owned()),
            city: Set("Madison".to_owned()),
            telephone: Set("6085559435".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        owner::ActiveModel {
            first_name: Set("Carlos".to_owned()), 
            last_name: Set("Estaban".to_owned()), 
            address: Set("2335 Independence La.".to_owned()),
            city: Set("Waunakee".to_owned()),
             telephone: Set("6085555487".to_owned()),
             ..Default::default()
        }
        .insert(db)
        .await?;


        Ok(())
    }
}
