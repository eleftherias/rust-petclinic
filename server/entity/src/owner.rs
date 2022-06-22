use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "owners")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub city: String,
    pub telephone: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pet::Entity")]
    Pet,
}

impl Related<super::pet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}