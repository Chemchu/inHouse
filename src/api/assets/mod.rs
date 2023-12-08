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
        .route("/_assets/compressed/*path", get(assets_handler_compressed))
        .nest_service(
            "/_assets/icons",
            ServeDir::new("src/assets/icons").precompressed_br(),
        )
        .nest_service(
            "/_assets/pwa/screenshots",
            ServeDir::new("src/assets/pwa/screenshots").precompressed_br(),
        )
        .nest_service(
            "/_assets/fonts",
            ServeDir::new("src/assets/fonts").precompressed_br(),
        )
}

static STYLE_CSS: &str = include_str!("../../assets/style.css");
static FAVICON: &str = include_str!("../../assets/favicon.svg");
static HTMX: &str = include_str!("../../assets/htmx.min.js");
static HTMX_BR: &[u8; 14064] = include_bytes!("../../assets/compressed/htmx.min.js.br");
static HTMX_MORPH: &str = include_str!("../../assets/htmx.morph.min.js");
static HTMX_MORPH_BR: &[u8; 162] = include_bytes!("../../assets/compressed/htmx.morph.min.js.br");
static ALPINE: &str = include_str!("../../assets/alpine.min.js");
static ALPINE_BR: &[u8; 14206] = include_bytes!("../../assets/compressed/alpine.min.js.br");
static ALPINE_PERSIST: &str = include_str!("../../assets/alpine.persist.min.js");
static ALPINE_PERSIST_BR: &[u8; 402] =
    include_bytes!("../../assets/compressed/alpine.persist.min.js.br");
static ALPINE_MORPH: &str = include_str!("../../assets/alpine.morph.min.js");
static ALPINE_MORPH_BR: &[u8; 1492] =
    include_bytes!("../../assets/compressed/alpine.morph.min.js.br");
static MANIFEST: &str = include_str!("../../assets/pwa/manifest.json");
static NONE: &str = "";

async fn assets_handler(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "axum".parse().unwrap());
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
        "htmx.morph.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, HTMX_MORPH)
        }
        "alpine.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, ALPINE)
        }
        "alpine.persist.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, ALPINE_PERSIST)
        }
        "alpine.morph.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, ALPINE_MORPH)
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

async fn assets_handler_compressed(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::SERVER, "axum".parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "max-age=31536000".parse().unwrap());
    headers.insert(header::CONTENT_ENCODING, "br".parse().unwrap());

    match path.as_str() {
        "htmx.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, HTMX_BR.into_response())
        }
        "htmx.morph.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, HTMX_MORPH_BR.into_response())
        }
        "alpine.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, ALPINE_BR.into_response())
        }
        "alpine.persist.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, ALPINE_PERSIST_BR.into_response())
        }
        "alpine.morph.min.js" => {
            headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            (StatusCode::OK, headers, ALPINE_MORPH_BR.into_response())
        }
        _ => {
            headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
            (StatusCode::NOT_FOUND, headers, NONE.into_response())
        }
    }
}
