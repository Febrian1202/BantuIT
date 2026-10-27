use axum::{extract::State, http::StatusCode};
use serde::Serialize;
use tokio::time::Instant;

use crate::{
    common::{ApiResponse, envelope::ApiResult},
    interface::AppState,
};

#[derive(Debug, Serialize)]
pub struct DatabaseHealth {
    pub status: String,
    pub latency_ms: u128,
    pub active_connections: u32,
    pub idle_connections: usize,
}

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: &'static str,
    pub environment: String,
    pub uptime_seconds: u64,
    pub database: DatabaseHealth,
}

pub async fn health_check(State(state): State<AppState>) -> ApiResult<HealthStatus> {
    let db_start = Instant::now();
    let (db_status, is_db_healthy) = match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => ("CONNECTED".to_string(), true),
        Err(e) => (format!("DISCONNECTED: {}", e), false),
    };
    let db_latency = db_start.elapsed().as_millis();

    let is_overall_ok = is_db_healthy;
    let status_code = if is_overall_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let body = HealthStatus {
        status: if is_overall_ok {
            "OK".to_string()
        } else {
            "DEGRADED".to_string()
        },
        version: env!("CARGO_PKG_VERSION"),
        environment: state.config.env.clone(),
        uptime_seconds: state.started_at.elapsed().as_secs(),
        database: DatabaseHealth {
            status: db_status,
            latency_ms: db_latency,
            active_connections: state.pool.size(),
            idle_connections: state.pool.num_idle(),
        },
    };

    let message = if is_overall_ok {
        "Sistem Beroperasi Normal"
    } else {
        "Layanan mengalami penurunan performa"
    };

    Ok(ApiResponse::with_status_code(
        body,
        message,
        status_code,
        is_overall_ok,
    ))
}
