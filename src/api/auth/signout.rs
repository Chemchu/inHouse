use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

pub async fn signout_handler(mut headers: HeaderMap) -> impl IntoResponse {
    headers.insert(
        header::SET_COOKIE,
        "sb:token=; Path=/; HttpOnly; Max-Age=0".parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        "sb:refresh=; Path=/; HttpOnly; Max-Age=0".parse().unwrap(),
    );
    headers.insert("HX-Redirect", "/login".parse().unwrap());

    (StatusCode::SEE_OTHER, headers).into_response()
}
