use crate::{components::sign_up_message::SignUpSuccessMessage, domain::AppState};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "signup.html")]
struct SignupTemplate {
    translator: crate::localization::Translator,
}

pub async fn signup_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = SignupTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn signup_handler(State(state): State<AppState>) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_millis(3000)).await;

    let template = SignUpSuccessMessage {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
