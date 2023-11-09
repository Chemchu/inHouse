use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use chrono::Datelike;
use reqwest::StatusCode;

use crate::{domain::AppState, localization::Translator};

#[derive(Template)]
#[template(path = "dashboard/index.html")]
struct DashboardHomeTemplate {
    translator: Translator,
    current_year: i32,
}

pub async fn dashboard_home_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = DashboardHomeTemplate {
        translator: state.translator.clone(),
        current_year: chrono::Utc::now().year(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
