use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub async fn index_html() -> impl IntoResponse {
    let template = HomeTemplate {};
    // let r_html = template.render().unwrap();
    let reply_html = askama::Template::render(&template).unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
