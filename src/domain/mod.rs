use crate::localization::Translator;

pub mod entity;

#[derive(Clone)]
pub struct AppState {
    pub supabase_url: String,
    pub supabase_api_key: String,
    pub conn: sea_orm::DatabaseConnection,
    pub translator: Translator,
}
