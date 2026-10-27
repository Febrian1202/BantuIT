use std::{env, error::Error};

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
}

impl DatabaseConfig {
    /**
     * Load konfigurasi DB dari .env atau pake nilai default
     */
    pub fn from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT").map_or(Ok(5432), |v| v.parse::<u16>())?;
        let user = env::var("DB_USER").unwrap_or_else(|_| "BantuIT".to_string());
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "root".to_string());
        let name = env::var("DB_NAME").unwrap_or_else(|_| "bantuit".to_string());

        let max_conn = env::var("DB_MAX_CONNECTIONS").map_or(Ok(10), |v| v.parse::<u32>())?;
        let min_conn = env::var("DB_MIN_CONNECTIONS").map_or(Ok(2), |v| v.parse::<u32>())?;

        Ok(Self {
            host,
            port,
            user,
            password,
            name,
            db_max_connections: max_conn,
            db_min_connections: min_conn,
        })
    }

    /**
     * Merangkai string database url untuk membuat pooling SQLx
     */
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}
