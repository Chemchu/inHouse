mod components;
mod database;
mod domain;
mod layer;
mod localization;
mod pages;
mod services;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    dotenv::from_filename(format!(".env.{}", std::env::var("ENVIRONMENT").unwrap())).ok();

    tracing_subscriber::fmt().with_target(false).pretty().init();

    // TODO: add when you are using DB connections
    let db = database::connect_to_db().await.unwrap();
    let translator = localization::Translator::new("es_ES".to_string(), "locales");

    let state = domain::AppState {
        supabase_api_key: std::env::var("SUPABASE_API_KEY")
            .expect("SUPABASE_API_KEY environment variable not found!"),
        supabase_url: std::env::var("SUPABASE_URL")
            .expect("SUPABASE_URL environment variable not found!"),
        conn: db,
        translator,
    };

    let app = Router::new()
        .route("/", get(pages::home::home_page_handler))
        .merge(pages::auth::routes(state))
        .route(
            "/internal-error",
            get(pages::generic::internal_error::internal_error_page_handler),
        )
        .fallback_service(get(pages::generic::not_found::not_found_page_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .merge(pages::assets::routes());

    let addr_str = "127.0.0.1:3000";
    let addr = addr_str.parse::<SocketAddr>().unwrap();

    tracing::info!("Application started! Serving on {} 🚀", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
