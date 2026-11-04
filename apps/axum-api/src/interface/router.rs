use axum::{
    Router,
    routing::{get, post},
};

use crate::interface::{
    AppState,
    handlers::{auth_handler::login, health_handler::health_check},
    middleware::logging::create_trace_layer,
};

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login));

    Router::new()
        .nest("/api", public_routes)
        .layer(create_trace_layer())
        .with_state(state)
}
