use axum::{Router, routing::get};

use crate::interface::{
    AppState, handlers::health_handler::health_check, middleware::logging::create_trace_layer,
};

pub fn create_router(state: AppState) -> Router {
    // let api_routes = Router::new();

    Router::new()
        .route("/health", get(health_check))
        .layer(create_trace_layer())
        .with_state(state)
}
