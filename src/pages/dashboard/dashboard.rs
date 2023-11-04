use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use reqwest::StatusCode;

use crate::{domain::AppState, localization::Translator};

#[derive(Template)]
#[template(path = "dashboard/index.html")]
struct DashboardTemplate {
    translator: Translator,
}

pub async fn dashboard_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = DashboardTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
