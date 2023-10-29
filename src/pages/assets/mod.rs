use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/_assets/*path", get(assets_handler))
}

static STYLE_CSS: &str = include_str!("../../../assets/style.css");
static FAVICON: &str = include_str!("../../../assets/favicon.svg");
static HTMX: &str = include_str!("../../../assets/htmx.min.js");
static LOADER: &str = include_str!("../../../assets/loader.svg");
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
        "loader.svg" => {
            headers.insert(header::CONTENT_TYPE, "image/svg+xml".parse().unwrap());
            (StatusCode::OK, headers, LOADER)
        }
        _ => {
            headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
            (StatusCode::NOT_FOUND, headers, NONE)
        }
    }
}
