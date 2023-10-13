mod pages;

use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/_assets/*path", get(handle_assets))
        .route("/", get(pages::home::home_page))
        .route("/products", get(pages::product::product_page));

    let addr_str = "127.0.0.1:3000";
    let addr = addr_str.parse::<SocketAddr>().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

static STYLE_CSS: &str = include_str!("../assets/style.css");
static FAVICON: &str = include_str!("../assets/favicon.svg");
static HTMX: &str = include_str!("../assets/htmx.min.js");
static NONE: &str = "";

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
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
