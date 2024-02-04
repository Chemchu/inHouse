use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Producto {
    pub nombre: String,
    pub cantidad: i32,
    pub barcode: Option<String>,
}
