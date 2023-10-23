use axum::{routing::get, Router};

pub mod login;
pub mod recover_account;
pub mod signup;

pub fn routes() -> Router {
    Router::new()
        // TODO: fix this
        // .route("/login", get(login::login_page_handler))
        .route("/sign-up", get(signup::signup_page_handler))
        .route(
            "/recover-account",
            get(recover_account::recover_account_page_handler),
        )
}
