use axum::{http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;

pub async fn signout_handler(mut headers: HeaderMap) -> impl IntoResponse {
    headers.insert(
        "Set-Cookie",
        "sb:token=; Path=/; HttpOnly; Max-Age=0".parse().unwrap(),
    );
    headers.insert("HX-Redirect", "/login".parse().unwrap());

    (StatusCode::SEE_OTHER, headers).into_response()
}
