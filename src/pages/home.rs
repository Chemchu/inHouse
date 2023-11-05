use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{domain::AppState, localization::Translator};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    translator: Translator,
}

pub async fn home_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = HomeTemplate {
        translator: state.translator.clone(),
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
