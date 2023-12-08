use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use reqwest::StatusCode;
use service::Token;

#[derive(Template)]
#[template(path = "dashboard/payments/index.html")]
struct PaymentsTemplate {
    translator: i18n::Translator,
    email: String,
    path: String,
}

pub async fn payments_page_handler(
    State(state): State<service::AppState>,
    Token(claims): Token,
) -> impl IntoResponse {
    let template = PaymentsTemplate {
        translator: state.translator.clone(),
        email: claims.email,
        path: "/dashboard/payments".to_string(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
