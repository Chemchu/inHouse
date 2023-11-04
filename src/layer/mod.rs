use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use reqwest::StatusCode;

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

pub async fn check_auth<A>(headers: HeaderMap, request: Request<A>, next: Next<A>) -> Response {
    match headers.get("cookie") {
        Some(cookies) => {
            tracing::info!(
                "User already logged in. Cookies: {}",
                cookies.to_str().unwrap()
            );

            let mut req = request;

            req.headers_mut()
                .insert("HX-Redirect", "/dashboard".parse().unwrap());

            next.run(req).await
        }
        None => {
            tracing::info!("User not logged in!");
            next.run(request).await
        }
    }
}

pub async fn check_logged_user<A>(
    headers: HeaderMap,
    request: Request<A>,
    next: Next<A>,
) -> Result<Response, StatusCode> {
    match headers.get("cookie") {
        Some(cookies) => Ok(next.run(request).await),
        None => {
            tracing::info!("User not logged in!");
            let mut req = request;

            req.headers_mut()
                .insert("HX-Redirect", "/login".parse().unwrap());

            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
