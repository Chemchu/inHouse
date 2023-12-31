use axum::{middleware, routing::get, Router};

use super::layer;

pub mod login;
pub mod recover_account;
pub mod signout;
pub mod signup;

pub fn routes(state: service::AppState) -> Router {
    Router::new()
        .route(
            "/login",
            get(login::login_page_handler).post(login::login_handler),
        )
        .route(
            "/sign-up",
            get(signup::signup_page_handler).post(signup::signup_handler),
        )
        .route(
            "/recover-account",
            get(recover_account::recover_account_page_handler),
        )
        .layer(middleware::from_fn(layer::check_auth))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            layer::inject_localization,
        ))
        .with_state(state)
        .route("/sign-out", axum::routing::post(signout::signout_handler))
}
