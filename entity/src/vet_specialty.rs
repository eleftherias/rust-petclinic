use sea_orm::entity::prelude::*;
use sea_orm::EnumIter;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "vet_specialties")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub vet_id: i32,
    #[sea_orm(primary_key)]
    pub specialty_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Vet,
    Specialty,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Vet => Entity::belongs_to(super::vet::Entity)
                .from(Column::VetId)
                .to(super::vet::Column::Id)
                .into(),
            Self::Specialty => Entity::belongs_to(super::specialty::Entity)
                .from(Column::SpecialtyId)
                .to(super::specialty::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
