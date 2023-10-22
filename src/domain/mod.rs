mod entities;

#[derive(Clone)]
pub struct AppState {
    pub conn: sea_orm::DatabaseConnection,
    pub translator: fn(key: &str) -> String,
}
