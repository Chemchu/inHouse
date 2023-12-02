use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use reqwest::StatusCode;
use service::Token;

#[derive(Template)]
#[template(path = "dashboard/settings.html")]
struct SettingsTemplate {
    translator: i18n::Translator,
    email: String,
}

pub async fn settings_page_handler(
    State(state): State<service::AppState>,
    Token(claims): Token,
) -> impl IntoResponse {
    let template = SettingsTemplate {
        translator: state.translator.clone(),
        email: claims.email,
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
