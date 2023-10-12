mod pages;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use pages::index::IndexTemplate;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_main));

    let addr_str = "127.0.0.1:3000";
    let addr = addr_str.parse::<SocketAddr>().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_main() -> impl IntoResponse {
    let template = IndexTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
