mod database;
mod domain;
mod localization;
mod pages;

#[macro_use]
extern crate rust_i18n;

use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

static EMPTY: &str = "";

#[tokio::main]
async fn main() {
    i18n!("locales", fallback = "es_ES");

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // let db = database::connect_to_db().await.unwrap();
    // let state: domain::AppState = domain::AppState { conn: db }; // Comment if you are not using it

    let app = Router::new()
        .route("/", get(pages::home::home_page_handler))
        .route("/products", get(pages::product::product_page_handler))
        .nest(EMPTY, pages::auth::routes())
        .fallback_service(get(pages::not_found::not_found_page_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .route("/_assets/*path", get(assets_handler));
    // .with_state(state); // Disable if you are not using DB connections

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
