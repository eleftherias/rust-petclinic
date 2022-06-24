use entity::specialty;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000004_index_specialties_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                sea_query::Index::create()
                    .name("idx_specialty_name")
                    .table(specialty::Entity)
                    .col(specialty::Column::Name)
                    .to_owned(),
            )
            .await
    }
}
