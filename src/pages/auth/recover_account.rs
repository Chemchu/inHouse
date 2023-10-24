use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::domain::AppState;

#[derive(Template)]
#[template(path = "recover_account.html")]
struct RecoverAccountTemplate {
    f: crate::localization::Translator,
}

pub async fn recover_account_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    // let template = RecoverAccountTemplate {
    //     translate: |key| state.translator.translate(key, None),
    // };

    let template = RecoverAccountTemplate {
        f: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
