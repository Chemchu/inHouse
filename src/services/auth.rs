use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::domain;
use crate::domain::entities::user::{self, Entity as User};

pub async fn exists_by_email(state: &domain::AppState, email: &str) -> Result<bool, DbErr> {
    let user = User::find()
        .filter(user::Column::Email.contains(email))
        .one(&state.conn)
        .await;

    match user {
        Ok(user) => Ok(user.is_some()),
        Err(e) => Err(e),
    }
}
