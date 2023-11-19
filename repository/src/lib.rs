use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

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

    // match initialize_schema(&db).await {
    //     Ok(_) => {}
    //     Err(err) => {
    //         tracing::error!("Error initializing database schema: {:?}", err);
    //         std::process::exit(1);
    //     }
    // }

    Ok(db)
}

async fn initialize_schema(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    tracing::info!("Initializing database schema...");

    let backend = db.get_database_backend();
    let schema = sea_orm::Schema::new(backend);
    let table_create_statement = schema.create_table_from_entity(entity::user_metadata::Entity);
    match db.execute(backend.build(&table_create_statement)).await {
        Ok(_) => {
            tracing::info!("Database schema initialized!");
            Ok(())
        }
        Err(err) => {
            tracing::error!("Error creating table: {:?}", err);
            std::process::exit(1);
        }
    }
}
