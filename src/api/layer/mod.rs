use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use service::AppState;

pub async fn inject_localization(
    State(state): State<service::AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let mut translator = state.translator.clone();

    translator.locale = i18n::get_locale_from_headers(&headers, translator.languages.clone());

    next.run(request).await
}

/// This middleware function checks if the user is logged in and handle the request accordingly.
///
/// * If it is, redirect to /dashboard.
/// * If it isn't, continue with the request.
///
/// This middleware function is commonly used in the login and register routes to prevent the user
/// to login again if it is already logged in.
pub async fn check_auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    match headers.get(header::COOKIE) {
        Some(cookies) => match check_authentication_cookies(cookies.to_str().unwrap()) {
            Some(Cookies::Token(_)) => {
                tracing::info!(
                    "User already logged in. Cookies: {}",
                    cookies.to_str().unwrap()
                );
                tracing::info!("Redirecting to /dashboard...");

                Err(Response::builder()
                    .header(header::LOCATION, "/dashboard")
                    .status(StatusCode::SEE_OTHER)
                    .body(Body::empty())
                    .unwrap())
            }
            Some(Cookies::Refresh(_)) => {
                tracing::info!(
                    "User already logged in. Cookies: {}",
                    cookies.to_str().unwrap()
                );
                tracing::info!("Redirecting to /dashboard...");

                Err(Response::builder()
                    .header(header::LOCATION, "/dashboard")
                    .status(StatusCode::SEE_OTHER)
                    .body(Body::empty())
                    .unwrap())
            }
            None => Ok(next.run(request).await),
        },
        None => Ok(next.run(request).await),
    }
}

/// This middleware function checks if the user is logged in and handle the request accordingly.
///
/// * If it is, continue with the request.
/// * If it isn't, try to refresh the session.
/// * If the refresh is successful, continue with the request.
/// * If the refresh fails, redirect to /login.
///
/// This middleware function is commonly used in the dashboard routes.
pub async fn check_logged_user(
    State(state): State<service::AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    match headers.get(header::COOKIE) {
        Some(cookies) => match check_authentication_cookies(cookies.to_str().unwrap()) {
            Some(Cookies::Token(_)) => {
                tracing::info!(
                    "User already logged in. Cookies: {}",
                    cookies.to_str().unwrap()
                );

                Ok(next.run(request).await)
            }
            Some(Cookies::Refresh(refresh_token)) => {
                tracing::info!("User not logged in! Trying to refresh session...");
                match refresh_session_tokens(&state, &refresh_token).await {
                    Ok(refresh_response) => {
                        tracing::info!("Session refreshed successfully!");

                        // This adds the fetched cookies to the request
                        request
                            .headers_mut()
                            .extend(refresh_response.server_cookies());
                        let mut response = next.run(request).await;

                        // This adds the fetched cookies to the response
                        response
                            .headers_mut()
                            .extend(refresh_response.browser_cookies());
                        Ok(response)
                    }
                    Err(e) => {
                        tracing::error!("Error refreshing session: {:?}", e);
                        Err(Response::builder()
                            .header(header::LOCATION, "/login")
                            .status(StatusCode::SEE_OTHER)
                            .body(Body::empty())
                            .unwrap())
                    }
                }
            }
            None => {
                tracing::info!("User not logged in! Redirecting to /login...");
                Err(Response::builder()
                    .header(header::LOCATION, "/login")
                    .status(StatusCode::SEE_OTHER)
                    .body(Body::empty())
                    .unwrap())
            }
        },
        None => {
            tracing::info!("User not logged in! Redirecting to /login...");
            Err(Response::builder()
                .header(header::LOCATION, "/login")
                .status(StatusCode::SEE_OTHER)
                .body(Body::empty())
                .unwrap())
        }
    }
}

/// This function checks if the cookies contains a token or a refresh token.
///
/// * If it contains a token, return the token.
/// * If it contains a refresh token, return the refresh token.
/// * If it doesn't contain any of them, return None.
fn check_authentication_cookies(cookies: &str) -> Option<Cookies> {
    if cookies.contains("sb:token=") {
        Some(Cookies::Token(
            cookies.split("sb:token=").collect::<Vec<&str>>()[1]
                .split(';')
                .collect::<Vec<&str>>()[0]
                .to_string(),
        ))
    } else if cookies.contains("sb:refresh=") {
        Some(Cookies::Refresh(
            cookies.split("sb:refresh=").collect::<Vec<&str>>()[1]
                .split(';')
                .collect::<Vec<&str>>()[0]
                .to_string(),
        ))
    } else {
        None
    }
}

/// This function refreshes the session tokens.
/// It returns a HeaderMap with the new cookies.
/// If the refresh fails, it returns an error.
async fn refresh_session_tokens(
    state: &AppState,
    refresh_token: &str,
) -> Result<RefreshResponse, reqwest::Error> {
    let body = json!({
        "refresh_token": refresh_token,
    });

    let response = reqwest::Client::new()
        .post(format!("{}/auth/v1/token", &state.supabase_url))
        .query(&[("grant_type", "refresh_token")])
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await; // Actualiza el token de la sesion

    match response {
        Ok(response) => Ok(response.json::<RefreshResponse>().await.unwrap()),
        Err(e) => {
            tracing::error!("Error refreshing session: {:?}", e);
            Err(e)
        }
    }
}

enum Cookies {
    Token(String),
    Refresh(String),
}

#[derive(Deserialize, Serialize, Debug)]
struct RefreshResponse {
    access_token: String,
    expires_in: u32,
    expires_at: u32,
    refresh_token: String,
}

/// This function returns a HeaderMap with the new cookies.
/// If the refresh fails, it returns an error.
/// If the refresh is successful, it returns a HeaderMap with the new cookies.
impl RefreshResponse {
    fn server_cookies(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            header::COOKIE,
            format!(
                "sb:token={}; Max-Age={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                self.access_token, self.expires_in
            )
            .parse()
            .unwrap(),
        );

        header_map.append(
            header::COOKIE,
            format!(
                "sb:refresh={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                self.refresh_token
            )
            .parse()
            .unwrap(),
        );

        header_map
    }

    fn browser_cookies(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            header::SET_COOKIE,
            format!(
                "sb:token={}; Max-Age={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                self.access_token, self.expires_in
            )
            .parse()
            .unwrap(),
        );

        header_map.append(
            header::SET_COOKIE,
            format!(
                "sb:refresh={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                self.refresh_token
            )
            .parse()
            .unwrap(),
        );

        header_map
    }
}
