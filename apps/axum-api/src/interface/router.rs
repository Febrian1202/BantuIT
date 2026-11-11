use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post, put},
};

use crate::interface::{
    AppState,
    handlers::{
        admin::ticket_category_handler::index,
        auth::{
            auth_handler::{login, logout},
            profile_handler::{show, update, update_password},
        },
        health_handler::health_check,
    },
    middleware::{
        logging::create_trace_layer,
        password_check::ensure_password_checked,
        rate_limiter::{rate_limit_api, rate_limit_login},
    },
};

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new().route("/health", get(health_check)).route(
        "/login",
        post(login).route_layer(from_fn_with_state(state.clone(), rate_limit_login)),
    );

    let auth_basic_routes = Router::new()
        .route("/logout", post(logout))
        .route("/me/password", put(update_password));

    let protected_routes = Router::new()
        .route("/me", get(show).put(update))
        .merge(ticket_category_routes())
        .layer(from_fn_with_state(state.clone(), ensure_password_checked));

    let api_routes = Router::new()
        .merge(public_routes)
        .merge(auth_basic_routes)
        .merge(protected_routes);

    Router::new()
        .nest("/api", api_routes)
        .layer(from_fn_with_state(state.clone(), rate_limit_api))
        .layer(create_trace_layer())
        .with_state(state)
}

fn ticket_category_routes() -> Router<AppState> {
    Router::new().route("/ticket-categories", get(index))
}
