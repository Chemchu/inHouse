use chrono::{DateTime, FixedOffset};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Vivienda {
    pub nombre: String,
    pub calle: String,
    pub numer_calle: String,
    pub numero_apartamento: Option<String>,
    pub tipo_vivienda: Option<String>,
    pub codigo_postal: String,
    pub ciudad: String,
    pub estado: String,
    pub pais: String,
    pub latitud: f64,
    pub longitud: f64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub informacion_adicional: Option<String>,
}
