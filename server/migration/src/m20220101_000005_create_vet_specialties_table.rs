use entity::vet_specialty;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Schema},
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000005_create_vet_specialties_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        let stmt = builder.build(&schema.create_table_from_entity(vet_specialty::Entity));
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
