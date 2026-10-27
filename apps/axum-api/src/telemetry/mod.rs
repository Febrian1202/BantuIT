use std::env;

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "bantuit_api=debug,tower_http=info,sqlx=warn".into());

    let is_production = env::var("APP_ENV").unwrap_or_default() == "production";

    if is_production {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    }
}
