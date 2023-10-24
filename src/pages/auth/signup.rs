use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{domain::AppState, localization::Translator};

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate {
    translator: Translator,
}

pub async fn signup_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = SignupTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
