use crate::components::auth::sign_up_message::{
    SignUpFailMessage, SignUpServerErrorMessage, SignUpSuccessMessage,
};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpForm {
    email: String,
    password: String,
    retry_password: String,
}

#[derive(Template)]
#[template(path = "auth/signup/signup.html")]
struct SignupTemplate {
    translator: i18n::Translator,
}

pub async fn signup_page_handler(State(state): State<service::AppState>) -> impl IntoResponse {
    let template = SignupTemplate {
        translator: state.translator.clone(),
    };

    let reply_html = askama::Template::render(&template).unwrap();

    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn signup_handler(
    State(state): State<service::AppState>,
    Form(payload): Form<SignUpForm>,
) -> impl IntoResponse {
    let correct_values = validate_form(&payload);
    if !correct_values {
        tracing::info!("Sign up form validation failed ⚠");
        let template = SignUpFailMessage {
            translator: state.translator.clone(),
        };

        let reply_html = askama::Template::render(&template).unwrap();

        return (StatusCode::OK, Html(reply_html).into_response());
    }

    let email_in_use = repository::auth::exists_by_email(&state.conn, &payload.email).await;
    match email_in_use {
        Ok(is_email_used) => {
            if is_email_used {
                tracing::info!("Email already in use ⚠");

                let template = SignUpFailMessage {
                    translator: state.translator.clone(),
                };

                let reply_html = askama::Template::render(&template).unwrap();

                return (StatusCode::OK, Html(reply_html).into_response());
            }
        }
        Err(e) => {
            tracing::error!("Database Error ❌: {}", e);

            let template = SignUpServerErrorMessage {
                translator: state.translator.clone(),
            };

            let reply_html = askama::Template::render(&template).unwrap();

            return (StatusCode::OK, Html(reply_html).into_response());
        }
    };

    let mut body = std::collections::HashMap::new();
    body.insert("email", &payload.email);
    body.insert("password", &payload.password);

    let response = reqwest::Client::new()
        .post(format!("{}/auth/v1/signup", &state.supabase_url))
        .header("apikey", &state.supabase_api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match response {
        Ok(response) => {
            let status = response.status();
            let text = response.text().await.unwrap();

            tracing::debug!("Status: {}", status);
            tracing::debug!("Text: {}", text);

            let template = SignUpSuccessMessage {
                translator: state.translator.clone(),
            };

            let reply_html = askama::Template::render(&template).unwrap();

            (StatusCode::OK, Html(reply_html).into_response())
        }
        _ => {
            tracing::error!("No response");
            let template = SignUpFailMessage {
                translator: state.translator.clone(),
            };

            let reply_html = askama::Template::render(&template).unwrap();

            (StatusCode::OK, Html(reply_html).into_response())
        }
    }
}

fn validate_form(payload: &SignUpForm) -> bool {
    if payload.email.is_empty() {
        return false;
    }

    if payload.password.is_empty() {
        return false;
    }

    if payload.retry_password.is_empty() {
        return false;
    }

    if payload.password.len() < 8 {
        return false;
    }

    payload.password == payload.retry_password
}
