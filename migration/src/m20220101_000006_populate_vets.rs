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
        "m20220101_000006_populate_vets"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        let sql = r#"
        INSERT INTO vets (first_name, last_name) SELECT 'James', 'Carter' WHERE NOT EXISTS (SELECT * FROM vets WHERE id=1);
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await?;

        let sql2 = r#"
        INSERT INTO vets (first_name, last_name) SELECT 'Helen', 'Leary' WHERE NOT EXISTS (SELECT * FROM vets WHERE id=2);
        "#;
        let stmt2 = Statement::from_string(manager.get_database_backend(), sql2.to_owned());
        manager.get_connection().execute(stmt2).await?;

        let sql3 = r#"
        INSERT INTO specialties (name) SELECT 'radiology' WHERE NOT EXISTS (SELECT * FROM specialties WHERE name='radiology');
        "#;
        let stmt3 = Statement::from_string(manager.get_database_backend(), sql3.to_owned());
        manager.get_connection().execute(stmt3).await?;

        let sql4 = r#"
        INSERT INTO vet_specialties VALUES (2, 1) ON CONFLICT (vet_id, specialty_id) DO NOTHING;
        "#;
        let stmt4 = Statement::from_string(manager.get_database_backend(), sql4.to_owned());
        manager.get_connection().execute(stmt4).await?;

        Ok(())
    }
}
