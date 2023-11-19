use cached::proc_macro::cached;
use cached::TimedSizedCache;
use entity::users::{self, Entity as User};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use service::AppState;

#[cached(
    type = "TimedSizedCache<String, Result<bool, String>>",
    create = "{ TimedSizedCache::with_size_and_lifespan_and_refresh(100, 30, true) }",
    convert = r#"{ format!("{}", email) }"#,
    sync_writes = true
)]
pub async fn exists_by_email(state: &AppState, email: &str) -> Result<bool, String> {
    let user = User::find()
        .filter(users::Column::Email.contains(email))
        .one(&state.conn)
        .await;

    match user {
        Ok(user) => Ok(user.is_some()),
        Err(e) => Err(e.to_string()),
    }
}
