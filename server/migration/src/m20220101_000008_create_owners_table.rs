use entity::owner;
use sea_orm_migration::sea_orm::{ConnectionTrait, Schema};
use sea_orm_migration::prelude::*;
use sea_orm_migration::{
    MigrationName, MigrationTrait,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000008_create_owners_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        let stmt = builder.build(&schema.create_table_from_entity(owner::Entity));
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
