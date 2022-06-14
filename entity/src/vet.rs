use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "vets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl Related<super::specialty::Entity> for Entity {
    fn to() -> RelationDef {
        super::vet_specialty::Relation::Specialty.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::vet_specialty::Relation::Vet.def().rev())
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
