use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "generic/not_found.html")]
struct NotFoundTemplate {
    translator: i18n::Translator,
}

pub async fn not_found_page_handler(State(state): State<service::AppState>) -> impl IntoResponse {
    let template = NotFoundTemplate {
        translator: state.translator.clone(),
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::NOT_FOUND, Html(reply_html).into_response())
}
