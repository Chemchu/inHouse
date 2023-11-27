use axum::{
    body::Empty,
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use reqwest::{header, StatusCode};

pub async fn inject_localization<B>(
    State(state): State<service::AppState>,
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    let mut translator = state.translator.clone();

    translator.locale = i18n::get_locale_from_headers(&headers, translator.languages.clone());

    next.run(request).await
}

pub async fn check_auth<B>(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    match headers.get(header::COOKIE) {
        Some(cookies) => {
            if cookies.to_str().unwrap().contains("sb:token=") {
                tracing::info!(
                    "User already logged in. Cookies: {}",
                    cookies.to_str().unwrap()
                );
                tracing::info!("Redirecting to /dashboard...");

                Err(Response::builder()
                    .header(header::LOCATION, "/dashboard")
                    .status(StatusCode::SEE_OTHER)
                    .body(Empty::new())
                    .unwrap())
            } else {
                Ok(next.run(request).await)
            }
        }
        None => Ok(next.run(request).await),
    }
}

pub async fn check_logged_user<B>(
    headers: HeaderMap,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, impl IntoResponse> {
    match headers.get(header::COOKIE) {
        Some(cookies) => {
            if cookies.to_str().unwrap().contains("sb:token=") {
                tracing::info!(
                    "User already logged in. Cookies: {}",
                    cookies.to_str().unwrap()
                );

                Ok(next.run(request).await)
            } else {
                tracing::info!("User not logged in! Redirecting to /login...");
                Err(Response::builder()
                    .header(header::LOCATION, "/login")
                    .status(StatusCode::SEE_OTHER)
                    .body(Empty::new())
                    .unwrap())
            }
        }
        None => {
            tracing::info!("User not logged in! Redirecting to /login...");
            Err(Response::builder()
                .header(header::LOCATION, "/login")
                .status(StatusCode::SEE_OTHER)
                .body(Empty::new())
                .unwrap())
        }
    }
}
