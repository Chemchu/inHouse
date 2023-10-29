use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;

use crate::{domain::AppState, localization::Translator};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    translator: Translator,
}

pub async fn login_page_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = LoginTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

// TODO: Implement login_handler
pub async fn login_handler(State(state): State<AppState>, Form(payload): Form<LoginForm>) {
    let mut body = std::collections::HashMap::new();
    body.insert("email", &payload.email);
    body.insert("password", &payload.password);

    let login_response = reqwest::Client::new()
        .post(format!("{}/auth/v1/token", &state.supabase_url))
        .query(&[("grant_type", "password")])
        .header("apikey", &state.supabase_api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await; // Inicia sesion bien y devuelve el token

    tracing::info!("login_response: {:?}", login_response);

    // match login_response {
    //     Ok(response) => {
    //         let status = response.status();
    //         if status.is_success() {
    //             let body = response.text().await.unwrap();
    //             let token: serde_json::Value = serde_json::from_str(&body).unwrap();
    //             let token = token["access_token"].as_str().unwrap();
    //             let cookie = axum::http::Cookie::build("token", token)
    //                 .path("/")
    //                 .secure(true)
    //                 .http_only(true)
    //                 .finish();
    //             let mut response = axum::http::Response::new(());
    //             response.headers_mut().insert(
    //                 axum::http::header::SET_COOKIE,
    //                 cookie.to_string().parse().unwrap(),
    //             );
    //             response
    //                 .headers_mut()
    //                 .insert(axum::http::header::LOCATION, "/".parse().unwrap());
    //             *response.status_mut() = axum::http::StatusCode::FOUND;
    //             response
    //         } else {
    //             let mut response = axum::http::Response::new(());
    //             response
    //                 .headers_mut()
    //                 .insert(axum::http::header::LOCATION, "/login".parse().unwrap());
    //             *response.status_mut() = axum::http::StatusCode::FOUND;
    //             response
    //         }
    //     }
    //     _ => {
    //         let mut response = axum::http::Response::new(());
    //         response
    //             .headers_mut()
    //             .insert(axum::http::header::LOCATION, "/login".parse().unwrap());
    //         *response.status_mut() = axum::http::StatusCode::FOUND;
    //         response
    //     }
    // }
}
