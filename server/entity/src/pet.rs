use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "pets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub birth_date: Date,
    pub type_id: i32,
    pub owner_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pet_type::Entity",
        from = "Column::TypeId",
        to = "super::pet_type::Column::Id"
    )]
    PetType,
    #[sea_orm(
        belongs_to = "super::owner::Entity",
        from = "Column::OwnerId",
        to = "super::owner::Column::Id"
    )]
    Owner,
}

impl Related<super::pet_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PetType.def()
    }
}

impl Related<super::owner::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}