use axum::{
    Router,
    routing::{get, post},
};

use crate::interface::{
    AppState,
    handlers::{
        auth_handler::{login, logout},
        health_handler::health_check,
    },
    middleware::logging::create_trace_layer,
};

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login));

    let auth_basic_route = Router::new().route("/logout", post(logout));

    Router::new()
        .nest("/api", public_routes.merge(auth_basic_route))
        .layer(create_trace_layer())
        .with_state(state)
}
