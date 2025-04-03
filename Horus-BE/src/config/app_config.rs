// src/config.rs
use config::{Config as ConfigLoader, ConfigError, Environment as ConfigEnvironment};
use serde::Deserialize;
use std::path::Path;
use tracing::debug;

#[derive(Debug, Clone, Deserialize, Default)]
pub enum RunMode {
    #[default]
    Development,
    Production,
    Test,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub env: RunMode,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: String,
    pub workers: usize,
    pub cors_origin: String,
    pub jwt_secret: String,
    pub token_expiry: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: "0.0.0.0:3000".to_string(),
            workers: 2,
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string()),
            token_expiry: 3600,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: usize,
    #[serde(default = "default_db_timeout")]
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    #[serde(default = "default_redis_timeout")]
    pub timeout_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
            max_connections: 5,
            timeout_seconds: 30,
        }
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".to_string(),
            timeout_seconds: 30,
        }
    }
}

fn default_db_timeout() -> u64 {
    30
}
fn default_redis_timeout() -> u64 {
    30
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        // Try to load environment file based on APP_ENV
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
        let env_file = format!("environment/{}.env", env);
        debug!("APP_ENV: {}", env);
        debug!("Loading environment file: {}", env_file);

        // Load the specific environment file if it exists
        if Path::new(&env_file).exists() {
            dotenvy::from_path(env_file).ok();
        }

        // Also try to load base.env if it exists
        if Path::new("environment/base.env").exists() {
            dotenvy::from_path("environment/base.env").ok();
        }

        let config = ConfigLoader::builder()
            .add_source(ConfigEnvironment::with_prefix("APP").separator("__"))
            .build()?;

        // Debug the configuration
        debug!("Loading config: {:?}", config);

        let app_config = config.try_deserialize()?;
        debug!("Loaded config: {:?}", app_config);

        Ok(app_config)
    }
}
