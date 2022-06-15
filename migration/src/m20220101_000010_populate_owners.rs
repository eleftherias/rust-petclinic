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
        "m20220101_000009_populate_pets"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let sql = r#"
        INSERT INTO owners (first_name, last_name, address, city, telephone) SELECT 'George', 'Franklin', '110 W. Liberty St.', 'Madison', '6085551023' WHERE NOT EXISTS (SELECT * FROM owners WHERE id=1);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await?;

        let sql2 = r#"
        INSERT INTO owners (first_name, last_name, address, city, telephone) SELECT 'Betty', 'Davis', '638 Cardinal Ave.', 'Sun Prairie', '6085551749' WHERE NOT EXISTS (SELECT * FROM owners WHERE id=2);
        "#;
        let stmt2 = Statement::from_string(manager.get_database_backend(), sql2.to_owned());
        manager.get_connection().execute(stmt2).await?;

        Ok(())
    }
}
