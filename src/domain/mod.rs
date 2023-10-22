use crate::localization::Translator;

mod entities;

#[derive(Clone)]
pub struct AppState<'a> {
    pub conn: sea_orm::DatabaseConnection,
    pub translator: Translator<'a>,
}
