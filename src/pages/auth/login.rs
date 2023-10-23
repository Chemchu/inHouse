use std::{env::Args, sync::Arc};

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::domain::AppState;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    translate: fn(&'_ str) -> String,
}

pub async fn login_page_handler(State(state): State<Arc<AppState<'_>>>) -> impl IntoResponse {
    todo!();
    let template = LoginTemplate {
        // translate: state.translator.translate,
        translate: |arg| "test".to_string(),
    };
    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
