use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use i18n::Translator;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use regex::Regex;
use reqwest::{header, StatusCode};
use serde::{Deserialize, Serialize};

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
    // type Rejection = (StatusCode, &'static str);
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let jwt_key =
            std::env::var("JWT_SECRET").expect("JWT_SECRET environment variable not found!");

        if let Some(jwt) = parts.headers.get(header::COOKIE) {
            let pattern = r#"sb:token=([^;]+)"#;
            let regex = Regex::new(pattern).expect("Invalid regex pattern");

            // Find the first match in the cookie string
            if let Some(captures) = regex.captures(jwt.to_str().unwrap()) {
                // Extract the content of sb:token
                let sb_token_content = captures.get(1).map_or("", |m| m.as_str());

                let mut validator = Validation::new(Algorithm::HS256);
                validator.set_audience(&["authenticated"]);

                let claims = decode::<Claims>(
                    sb_token_content,
                    &DecodingKey::from_secret(jwt_key.clone().as_ref()),
                    &validator,
                );
                if claims.is_err() {
                    tracing::error!("Error decoding header: {:?}", claims.err().unwrap());
                    let mut header_map = header::HeaderMap::new();
                    header_map.insert(header::SET_COOKIE, "sb:token=; Max-Age=0".parse().unwrap());
                    header_map.append(
                        header::SET_COOKIE,
                        "sb:refresh=; Max-Age=0".parse().unwrap(),
                    );
                    header_map.append(header::LOCATION, "/login".parse().unwrap());

                    return Err((StatusCode::SEE_OTHER, header_map.into_response()).into_response());
                }

                Ok(Token(claims.unwrap().claims))
            } else {
                Err((StatusCode::UNAUTHORIZED, "Unauthorized").into_response())
            }
        } else {
            Err((StatusCode::UNAUTHORIZED, "Unauthorized").into_response())
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
pub struct UserMetadata {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Amr {
    pub method: String,
    pub timestamp: i64,
}
