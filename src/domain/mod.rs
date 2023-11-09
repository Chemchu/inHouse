use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::decode_header;
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

pub struct Token(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if let Some(jwt) = parts.headers.get(COOKIE) {
            let header = decode_header(jwt.to_str().unwrap());
            tracing::info!("Header: {:?}", header);

            // TODO: parse JWT into Claims
            // let token_data = match decode::<Claims>(
            //     &token,
            //     &DecodingKey::from_secret(key),
            //     &Validation::new(Algorithm::HS512),
            // )

            Ok(Token(Claims {
                aud: String::from(""),
                exp: 0,
                iat: 0,
                sub: String::from(""),
                email: String::from(""),
                phone: String::from(""),
                app_metadata: AppMetadata {
                    provider: String::from(""),
                    providers: vec![],
                },
                user_metadata: UserMetadata {},
                role: String::from(""),
                aal: String::from(""),
                amr: vec![],
                session_id: String::from(""),
            }))
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
