use axum::{middleware, routing::get, Router};

use crate::{domain::AppState, layer};

use self::groceries::groceries_page_handler;
use self::home::dashboard_home_page_handler;

pub mod groceries;
pub mod home;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_home_page_handler))
        .route("/dashboard/groceries", get(groceries_page_handler))
        .layer(middleware::from_fn(layer::check_logged_user))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            layer::inject_localization,
        ))
        .with_state(state)
}
