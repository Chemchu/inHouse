use cached::proc_macro::cached;
use cached::TimedSizedCache;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::domain;
use crate::domain::entity::users::{self, Entity as User};

#[cached(
    type = "TimedSizedCache<String, Result<bool, String>>",
    create = "{ TimedSizedCache::with_size_and_lifespan_and_refresh(100, 30, true) }",
    convert = r#"{ format!("{}", email) }"#,
    sync_writes = true
)]
pub async fn exists_by_email(state: &domain::AppState, email: &str) -> Result<bool, String> {
    let db_connection = std::sync::Arc::as_ref(&state.conn);
    let user = User::find()
        .filter(users::Column::Email.contains(email))
        .one(db_connection)
        .await;

    match user {
        Ok(user) => Ok(user.is_some()),
        Err(e) => Err(e.to_string()),
    }
}
