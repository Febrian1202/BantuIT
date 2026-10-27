pub mod config;
pub mod connection;

pub use config::DatabaseConfig;

pub type DB = config::DatabaseConfig;
