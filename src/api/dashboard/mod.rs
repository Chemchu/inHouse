use axum::{middleware, routing::get, Router};

use self::chats::chats_page_handler;
use self::groceries::groceries_page_handler;
use self::home::dashboard_home_page_handler;
use self::members::members_page_handler;
use self::payments::payments_page_handler;
use self::summary::summary_page_handler;

use super::layer;

pub mod chats;
pub mod groceries;
pub mod home;
pub mod members;
pub mod payments;
pub mod summary;
pub mod user_settings;

pub fn routes(state: service::AppState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_home_page_handler))
        .route("/dashboard/members", get(members_page_handler))
        .route("/dashboard/groceries", get(groceries_page_handler))
        .route("/dashboard/chats", get(chats_page_handler))
        .route("/dashboard/payments", get(payments_page_handler))
        .route("/dashboard/summary", get(summary_page_handler))
        .route(
            "/dashboard/settings",
            get(user_settings::settings_page_handler),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            layer::check_logged_user,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            layer::inject_localization,
        ))
        .with_state(state)
}
