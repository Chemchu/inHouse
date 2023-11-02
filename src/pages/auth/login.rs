use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Form,
};
use reqwest::{header, Error, Response};
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
pub async fn login_handler(
    headers: HeaderMap,
    State(state): State<AppState>,
    Form(payload): Form<LoginForm>,
) -> impl IntoResponse {
    if headers.contains_key(header::AUTHORIZATION) {
        tracing::debug!("User already logged in");
        return (
            StatusCode::FOUND,
            [(header::LOCATION, "/dashboard")].into_response(),
        );
    }

    match login(&state, &payload.email, &payload.password).await {
        Ok(response) => {
            tracing::info!("Supabase login successful: {:?}", response);
            let status = response.status();
            if status.is_success() {
                // Set the JWT token from response as a Authorization token
                let body = response.text().await.unwrap();
                let token: serde_json::Value = serde_json::from_str(&body).unwrap();
                (
                    StatusCode::FOUND,
                    [
                        (
                            header::AUTHORIZATION,
                            format!("Bearer {}", token["access_token"]),
                        ),
                        (header::LOCATION, "/dashboard".parse().unwrap()),
                    ]
                    .into_response(),
                )
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Html("Error al iniciar sesión").into_response(),
                )
            }
        }
        Err(response) => {
            tracing::debug!("Supabase login failed: {:?}", response);
            (
                StatusCode::UNAUTHORIZED,
                Html("Error al iniciar sesión").into_response(),
            )
        }
    }
}

async fn login(state: &AppState, email: &str, password: &str) -> Result<Response, Error> {
    let mut body = std::collections::HashMap::new();
    body.insert("email", email);
    body.insert("password", password);

    reqwest::Client::new()
        .post(format!("{}/auth/v1/token", &state.supabase_url))
        .query(&[("grant_type", "password")])
        .header("apikey", &state.supabase_api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await // Inicia sesion bien y devuelve el token
}
