use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario {
    pub username: String,
    pub email: String,
    pub email_confirmed_at: DateTime<FixedOffset>,
    pub nombre: Option<String>,
    pub apellidos: Option<String>,
    pub birthdate: Option<DateTime<FixedOffset>>,
    pub telefono: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub last_sign_in_at: Option<DateTime<FixedOffset>>,
    pub banned_until: Option<DateTime<FixedOffset>>,
}
