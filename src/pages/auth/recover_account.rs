use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};

use crate::{domain::AppState, localization::get_locale_from_headers};

#[derive(Template)]
#[template(path = "recover_account.html")]
struct RecoverAccountTemplate {
    translator: crate::localization::Translator,
}

pub async fn recover_account_page_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let mut translator = state.translator.clone();
    let locale = get_locale_from_headers(&headers, translator.languages.clone()).to_owned();

    translator.locale = todo!();
    // translator.locale = Box::leak(locale.into_boxed_str()); // ---> Probar sin eso ya que provoca memory leak

    let template = RecoverAccountTemplate { translator };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}
