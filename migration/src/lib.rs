pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_user;
mod m20231226_183754_create_table_vivienda;
mod m20240114_163635_create_table_recibo;
mod m20240114_170806_create_table_compras;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_user::Migration),
            Box::new(m20231226_183754_create_table_vivienda::Migration),
            Box::new(m20240114_163635_create_table_recibo::Migration),
            Box::new(m20240114_170806_create_table_compras::Migration),
        ]
    }
}
