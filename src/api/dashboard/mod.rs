use axum::{middleware, routing::get, Router};

use self::groceries::groceries_page_handler;
use self::home::dashboard_home_page_handler;

use super::layer;

pub mod groceries;
pub mod home;
pub mod user_settings;

pub fn routes(state: service::AppState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_home_page_handler))
        .route("/dashboard/groceries", get(groceries_page_handler))
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
