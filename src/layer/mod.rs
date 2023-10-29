use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};

use crate::domain::AppState;

pub async fn inject_localization<A>(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request<A>,
    next: Next<A>,
) -> Response {
    let mut translator = state.translator.clone();

    translator.locale =
        crate::localization::get_locale_from_headers(&headers, translator.languages.clone());

    next.run(request).await
}
