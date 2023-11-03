use cached::proc_macro::cached;
use cached::TimedSizedCache;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::domain;
use crate::domain::entities::user::{self, Entity as User};

// TODO: devolver un error en vez de un bool y validar fuera
#[cached(
    type = "TimedSizedCache<String, bool>",
    create = "{ TimedSizedCache::with_size_and_lifespan_and_refresh(100, 30, true) }",
    convert = r#"{ format!("{}", email) }"#,
    sync_writes = true
)]
pub async fn exists_by_email(state: &domain::AppState, email: &str) -> bool {
    let user = User::find()
        .filter(user::Column::Email.contains(email))
        .one(&state.conn)
        .await;

    match user {
        Ok(user) => user.is_some(),
        Err(_) => false,
    }
}
