use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post, put},
};

use crate::interface::{
    AppState,
    handlers::{
        auth_handler::{login, logout},
        health_handler::health_check,
        profile_handler::update_password,
    },
    middleware::{
        logging::create_trace_layer,
        rate_limiter::{rate_limit_api, rate_limit_login},
    },
};

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new().route("/health", get(health_check)).route(
        "/login",
        post(login).route_layer(from_fn_with_state(state.clone(), rate_limit_login)),
    );

    let auth_basic_route = Router::new()
        .route("/logout", post(logout))
        .route("/me/password", put(update_password));

    Router::new()
        .nest("/api", public_routes.merge(auth_basic_route))
        .layer(from_fn_with_state(state.clone(), rate_limit_api))
        .layer(create_trace_layer())
        .with_state(state)
}
