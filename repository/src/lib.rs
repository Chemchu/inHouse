use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod auth;

pub async fn connect_to_db() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let conn_url = std::env::var("DATABASE_URL");
    match &conn_url {
        Ok(conn_url) => {
            tracing::info!("Database URL: {}", conn_url);
        }
        Err(_) => {
            tracing::error!("DATABASE_URL environment variable not found!");
            std::process::exit(1);
        }
    }

    let mut opt = ConnectOptions::new(conn_url.unwrap());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;

    Ok(db)
}
