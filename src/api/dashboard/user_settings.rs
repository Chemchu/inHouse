use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::Datelike;
use reqwest::StatusCode;

use crate::{
    domain::{AppState, Token},
    util::localization::Translator,
};

#[derive(Template)]
#[template(path = "dashboard/settings.html")]
struct SettingsTemplate {
    translator: Translator,
    current_year: i32,
    email: String,
}

pub async fn settings_page_handler(
    State(state): State<AppState>,
    Token(claims): Token,
) -> impl IntoResponse {
    let template = SettingsTemplate {
        translator: state.translator.clone(),
        current_year: chrono::Utc::now().year(),
        email: claims.email,
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
