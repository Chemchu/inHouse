mod domain;
mod pages;

use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let db = db_connection_handler().await.unwrap();

    let app = Router::new()
        .route("/", get(pages::home::home_page_handler))
        .route("/products", get(pages::product::product_page_handler))
        .fallback_service(get(pages::not_found::not_found_page_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .route("/_assets/*path", get(assets_handler))
        .with_state(domain::AppState { conn: db });

    let addr_str = "127.0.0.1:3000";
    let addr = addr_str.parse::<SocketAddr>().unwrap();

    tracing::info!("Application started! Serving on {} 🚀", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

static STYLE_CSS: &str = include_str!("../assets/style.css");
static FAVICON: &str = include_str!("../assets/favicon.svg");
static HTMX: &str = include_str!("../assets/htmx.min.js");
static NONE: &str = "";

async fn assets_handler(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    match path.as_str() {
        "style.css" => {
            headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
            (StatusCode::OK, headers, STYLE_CSS)
        }
        "favicon.svg" => {
            headers.insert(header::CONTENT_TYPE, "image/svg+xml".parse().unwrap());
            (StatusCode::OK, headers, FAVICON)
        }
        "htmx.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, HTMX)
        }
        _ => {
            headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
            (StatusCode::NOT_FOUND, headers, NONE)
        }
    }
}

async fn db_connection_handler() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let conn_url = std::env::var("TURSO_URL");
    match &conn_url {
        Ok(conn_url) => {
            tracing::info!("Database URL: {}", conn_url);
        }
        Err(_) => {
            tracing::error!("TURSO_URL environment variable not found!");
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

    Ok(db)
}
