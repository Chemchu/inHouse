use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate;

pub async fn not_found_page_handler() -> impl IntoResponse {
    let template = NotFoundTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::NOT_FOUND, Html(reply_html).into_response())
}
