use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "specialties")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}


impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl Related<super::vet::Entity> for Entity {
    fn to() -> RelationDef {
        super::vet_specialty::Relation::Vet.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::vet_specialty::Relation::Specialty.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}