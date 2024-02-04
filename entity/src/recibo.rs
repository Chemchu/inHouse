use chrono::{DateTime, FixedOffset};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Recibo {
    pub descripcion: String,
    pub total: i32,
    pub created_at: DateTime<FixedOffset>,
}
