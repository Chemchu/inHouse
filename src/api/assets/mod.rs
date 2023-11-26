use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    Router::new()
        .route("/_assets/*path", get(assets_handler))
        .nest_service("/_assets/icons", ServeDir::new("src/assets/icons"))
        .nest_service(
            "/_assets/pwa/screenshots",
            ServeDir::new("src/assets/pwa/screenshots"),
        )
}

static STYLE_CSS: &str = include_str!("../../assets/style.css");
static FAVICON: &str = include_str!("../../assets/favicon.svg");
static HTMX: &str = include_str!("../../assets/htmx.min.js");
static HYPERSCRIPT: &str = include_str!("../../assets/hyperscript.min.js");
static MANIFEST: &str = include_str!("../../assets/pwa/manifest.json");
static NONE: &str = "";

async fn assets_handler(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CACHE_CONTROL, "max-age=31536000".parse().unwrap());

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
        "hyperscript.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, HYPERSCRIPT)
        }
        "manifest.json" => {
            headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
            (StatusCode::OK, headers, MANIFEST)
        }
        _ => {
            headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
            (StatusCode::NOT_FOUND, headers, NONE)
        }
    }
}
