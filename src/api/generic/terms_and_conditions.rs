use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "generic/terms_and_conditions.html")]
struct TermsAndConditionsTemplate {
    translator: i18n::Translator,
}

pub async fn terms_and_conditions_page_handler(
    State(state): State<service::AppState>,
) -> impl IntoResponse {
    let template = TermsAndConditionsTemplate {
        translator: state.translator.clone(),
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::NOT_FOUND, Html(reply_html).into_response())
}
