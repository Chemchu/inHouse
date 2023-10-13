use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "products.html")]
struct ProductTemplate;

pub async fn product_page() -> impl IntoResponse {
    let template = ProductTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
