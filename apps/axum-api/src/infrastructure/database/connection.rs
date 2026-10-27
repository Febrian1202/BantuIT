use std::time::Duration;

use sqlx::{PgPool, migrate::MigrateError, postgres::PgPoolOptions};
use tracing::info;

use crate::infrastructure::AppConfig;

/**
 * Untuk membuat Database Connection Pooling
 */
pub async fn create_pool(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
    info!("Menghubungkan ke database PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(config.db.db_max_connections)
        .min_connections(config.db.db_min_connections)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(&config.db.connection_string())
        .await?;

    info!("Berhasil terhubung ke database PostgreSQL...");
    Ok(pool)
}

/**
 * Helper untuk menjalankan database migrations
 */
pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrateError> {
    info!("Menjalankan migration database...");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("Migrasi database berhasil dijalankan...");
    Ok(())
}
