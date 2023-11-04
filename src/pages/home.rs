use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn home_page_handler() -> impl IntoResponse {
    let template = HomeTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
