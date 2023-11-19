use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use reqwest::{header, StatusCode};

use crate::domain::AppState;

pub async fn inject_localization<B>(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut translator = state.translator.clone();

    translator.locale =
        crate::util::localization::get_locale_from_headers(&headers, translator.languages.clone());

    next.run(request).await
}

pub async fn check_auth<B>(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    match headers.get("cookie") {
        Some(cookies) => {
            tracing::info!(
                "User already logged in. Cookies: {}",
                cookies.to_str().unwrap()
            );
            tracing::info!("Redirecting to /dashboard...");

            // let header = (header::LOCATION, "/dashboard");
            // Err((StatusCode::SEE_OTHER, header))

            Err(Response::builder()
                .header(header::LOCATION, "/dashboard")
                .status(StatusCode::SEE_OTHER)
                .body(http_body::Empty::new())
                .unwrap())
        }
        None => Ok(next.run(request).await),
    }
}

pub async fn check_logged_user<B>(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    match headers.get("cookie") {
        Some(_) => Ok(next.run(request).await),
        None => {
            tracing::info!("User not logged in! Redirecting to /login...");
            Err(Response::builder()
                .header(header::LOCATION, "/login")
                .status(StatusCode::SEE_OTHER)
                .body(http_body::Empty::new())
                .unwrap())
        }
    }
}
