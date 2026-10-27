use std::time::Instant;

use sqlx::PgPool;

use crate::infrastructure::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub started_at: Instant,
}

impl AppState {
    pub fn new(config: AppConfig, pool: PgPool, started_at: Instant) -> Self {
        Self {
            config,
            pool,
            started_at,
        }
    }
}
