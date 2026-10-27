use std::{env, error::Error};

use crate::infrastructure::database::DB;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub db: DB,
}

impl AppConfig {
    /**
     * Memuat konfigurasi Aplikasi dari .env, atau pake default value
     */
    pub fn from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("APP_PORT").map_or(Ok(8000), |v| v.parse::<u16>())?;
        let env = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
        let db = DB::from_env()?;

        Ok(Self {
            host,
            port,
            env,
            db,
        })
    }

    /**
     * Helper untuk display alamat server
     */
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /**
     * Helper untuk mengetahui sedang mode production atau bukan
     */
    pub fn is_production(&self) -> bool {
        self.env == "production"
    }
}
