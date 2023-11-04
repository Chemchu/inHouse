use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use reqwest::StatusCode;

use crate::{domain::AppState, localization::Translator};

#[derive(Template)]
#[template(path = "dashboard/home.html")]
struct LoginTemplate {
    translator: Translator,
}

pub async fn dashboard_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = LoginTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
