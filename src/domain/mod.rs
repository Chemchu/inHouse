use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue},
};
use reqwest::{header::COOKIE, StatusCode};
use serde::{Deserialize, Serialize};

use crate::localization::Translator;

pub mod entity;

#[derive(Clone)]
pub struct AppState {
    pub supabase_url: String,
    pub supabase_api_key: String,
    pub conn: sea_orm::DatabaseConnection,
    pub translator: Translator,
}

pub struct Token(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(user) = parts.headers.get(COOKIE) {
            Ok(Token(user.clone()))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Unauthorized"))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub sub: String,
    pub email: String,
    pub phone: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub role: String,
    pub aal: String,
    pub amr: Vec<Amr>,
    pub session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserMetadata {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Amr {
    pub method: String,
    pub timestamp: i64,
}
