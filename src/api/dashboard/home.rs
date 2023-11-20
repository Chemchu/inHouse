use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::Datelike;
use reqwest::StatusCode;
use service::Token;

#[derive(Template)]
#[template(path = "dashboard/index.html")]
struct DashboardHomeTemplate {
    translator: i18n::Translator,
    current_year: i32,
    email: String,
}

pub async fn dashboard_home_page_handler(
    State(state): State<service::AppState>,
    Token(claims): Token,
) -> impl IntoResponse {
    let template = DashboardHomeTemplate {
        translator: state.translator.clone(),
        current_year: chrono::Utc::now().year(),
        email: claims.email,
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
