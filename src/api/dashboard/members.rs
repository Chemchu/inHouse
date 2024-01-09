use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use service::Token;

#[derive(Template)]
#[template(path = "dashboard/members/index.html")]
struct MembersTemplate {
    translator: i18n::Translator,
    email: String,
    path: String,
}

pub async fn members_page_handler(
    State(state): State<service::AppState>,
    Token(claims): Token,
) -> impl IntoResponse {
    let template = MembersTemplate {
        translator: state.translator.clone(),
        email: claims.email,
        path: "/dashboard/members".to_string(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
