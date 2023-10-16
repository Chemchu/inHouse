use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate;

pub async fn signup_page_handler() -> impl IntoResponse {
    let template = SignupTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
