use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::domain::AppState;

#[derive(Template)]
#[template(path = "generic/not_found.html")]
struct NotFoundTemplate {
    translator: crate::util::localization::Translator,
}

pub async fn not_found_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = NotFoundTemplate {
        translator: state.translator.clone(),
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::NOT_FOUND, Html(reply_html).into_response())
}
