use axum::{Router, routing::get};

use crate::interface::{AppState, handlers::health_handler::health_check};

pub fn create_router(state: AppState) -> Router {
    // let api_routes = Router::new();

    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}
