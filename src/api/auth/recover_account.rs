use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "auth/login/recover_account.html")]
struct RecoverAccountTemplate {
    translator: i18n::Translator,
}

pub async fn recover_account_page_handler(
    State(state): State<service::AppState>,
) -> impl IntoResponse {
    let template = RecoverAccountTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
