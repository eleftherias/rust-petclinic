use entity::vet;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_index_vets_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                sea_query::Index::create()
                    .name("idx_vets_last_name")
                    .table(vet::Entity)
                    .col(vet::Column::LastName)
                    .to_owned(),
            )
            .await
    }
}
