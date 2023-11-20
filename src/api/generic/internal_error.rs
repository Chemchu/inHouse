use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "generic/internal_error.html")]
struct InternalErrorTemplate;

pub async fn internal_error_page_handler() -> impl IntoResponse {
    let template = InternalErrorTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
