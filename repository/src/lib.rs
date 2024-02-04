use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

pub mod auth;

pub async fn connect_to_db() -> surrealdb::Result<Surreal<Client>> {
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

    let db = Surreal::new::<Ws>(conn_url.unwrap()).await?;
    db.use_ns("public").use_db("public").await?;

    // Write custom migrator and add it here

    Ok(db)
}
