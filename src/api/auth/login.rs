use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Form,
};
use reqwest::{header, Error, Response};
use serde::{Deserialize, Serialize};

use crate::components::auth::login_failed_message::LoginFailedMessageTemplate;

#[derive(Template)]
#[template(path = "auth/login/login.html")]
struct LoginTemplate {
    translator: i18n::Translator,
}

pub async fn login_page_handler(State(state): State<service::AppState>) -> impl IntoResponse {
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

#[derive(Deserialize, Serialize, Debug)]
struct LoggedUser {
    id: String,
    aud: String,
    role: String,
    email: String,
    email_confirmed_at: String,
    phone: String,
    confirmed_at: String,
    last_sign_in_at: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    expires_at: u32,
    refresh_token: String,
    user: LoggedUser,
}

impl LoginResponse {
    fn headers(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();

        header_map.insert(
            header::SET_COOKIE,
            format!(
                "sb:token={}; Max-Age={}; sb:refresh_token={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                self.access_token, self.expires_in, self.refresh_token
            )
            .parse()
            .unwrap(),
        );

        tracing::info!("Headers: {:?}", header_map);

        header_map
    }
}

pub async fn login_handler(
    State(state): State<service::AppState>,
    Form(payload): Form<LoginForm>,
) -> impl IntoResponse {
    match login(&state, &payload.email, &payload.password).await {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                let body = response.text().await.unwrap();

                tracing::info!("Supabase login successful: {:?}", body);

                let login_response: LoginResponse = serde_json::from_str(&body).unwrap();

                tracing::info!("Supabase login successful: {:?}", login_response);

                let mut headers = login_response.headers();
                headers.insert("HX-Redirect", "/dashboard".parse().unwrap());
                (StatusCode::OK, headers.into_response())
            } else {
                tracing::info!("Supabase login failed: {:?}", response);
                let template = LoginFailedMessageTemplate {
                    translator: state.translator.clone(),
                };
                let reply_html = askama::Template::render(&template).unwrap();

                (StatusCode::OK, Html(reply_html).into_response())
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

async fn login(state: &service::AppState, email: &str, password: &str) -> Result<Response, Error> {
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
