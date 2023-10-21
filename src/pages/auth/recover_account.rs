use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "recover_account.html")]
struct RecoverAccountTemplate;

pub async fn recover_account_page_handler() -> impl IntoResponse {
    let template = RecoverAccountTemplate {};
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
