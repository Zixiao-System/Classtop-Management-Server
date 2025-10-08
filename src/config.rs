use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub app_version: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8765".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            app_version: env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
        })
    }
}
