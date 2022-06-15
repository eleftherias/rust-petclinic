use sea_orm_migration::sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::{
    prelude::*,
};
use sea_orm_migration::{
    MigrationName, MigrationTrait,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000011_populate_pets"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let sql = r#"
        INSERT INTO types (name) SELECT 'cat' WHERE NOT EXISTS (SELECT * FROM specialties WHERE name='cat');
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await?;

        let sql2 = r#"
        INSERT INTO pets (name, birth_date, type_id, owner_id) SELECT 'Leo', '2000-09-07', 1, 1 WHERE NOT EXISTS (SELECT * FROM pets WHERE id=1);
        "#;
        let stmt2 = Statement::from_string(manager.get_database_backend(), sql2.to_owned());
        manager.get_connection().execute(stmt2).await?;

        Ok(())
    }
}
