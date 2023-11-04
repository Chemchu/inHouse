use axum::{middleware, routing::get, Router};

use crate::{domain::AppState, layer};

use self::dashboard::dashboard_page_handler;

pub mod dashboard;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_page_handler))
        .layer(middleware::from_fn(layer::check_auth))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            layer::inject_localization,
        ))
        .with_state(state)
}
