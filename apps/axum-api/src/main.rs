use std::{error::Error, time::Instant};

use bantuit_api::{
    infrastructure::{
        AppConfig,
        database::connection::{create_pool, run_migrations},
    },
    interface::{AppState, router::create_router},
    telemetry::init_tracing,
};
use tokio::{self, signal};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    init_tracing();

    info!("Memulai BantuIT Axum Api...");
    let config = AppConfig::from_env()?;
    info!(
        "Konfigurasi berhasil dimuat: host={}, port={}, env={}",
        config.host, config.port, config.env
    );

    let pool = match create_pool(&config).await {
        Ok(p) => {
            // Jalankan migration jika pembuatan koneksi database berhasil
            if let Err(e) = run_migrations(&p).await {
                tracing::warn!("Gagal menjalankan auto-migrations: {}. Melanjutkan ...", e);
                return Err(e.into());
            }
            p
        }
        Err(e) => {
            tracing::error!("Tidak dapat terhubung ke PostgreSql: {}", e);
            return Err(e.into());
        }
    };
    info!("Koneksi database berhasil dibuat");

    // Axum State Management
    let state = AppState::new(config.clone(), pool, Instant::now());
    info!("Berhasil membuat State untuk Axum");

    // Setup Router
    let app = create_router(state);

    // Bind & Serve Axum
    let addr = config.server_address();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let display_host = if config.host == "0.0.0.0" {
        "localhost"
    } else {
        &config.host
    };
    info!(
        "Server Axum/BantuIT berhasil berjalan di http://{}:{}",
        display_host, config.port
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server Axum berhasil dihentikan secara aman (gracefull shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Gagal mengaktifkan Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Gagal mengaktifkan SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Menerima sinyal Ctrl+C, bersiap mematikan server...");
        },
        _ = terminate => {
            info!("Menerima sinyal SIGTERM, bersiap mematikan server...");
        },
    }
}
