use axum::{middleware, routing::get, Router};

use crate::{domain::AppState, layer};

pub mod login;
pub mod recover_account;
pub mod signup;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login::login_page_handler))
        .route("/sign-up", get(signup::signup_page_handler))
        .route(
            "/recover-account",
            get(recover_account::recover_account_page_handler),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            layer::inject_localization,
        ))
        .with_state(state)
}
