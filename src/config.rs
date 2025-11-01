use anyhow::Result;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    PostgreSQL,
    Mssql,
}

impl DatabaseType {
    pub fn parse(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "postgresql" | "postgres" => Ok(DatabaseType::PostgreSQL),
            "mssql" | "sqlserver" => Ok(DatabaseType::Mssql),
            _ => Err(anyhow::anyhow!("Unsupported database type: {}", s)),
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::PostgreSQL => "PostgreSQL",
            DatabaseType::Mssql => "SQL Server",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    #[allow(dead_code)]
    pub database_type: DatabaseType,
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub app_version: String,
    pub jwt_secret: String,
    pub cors_allowed_origins: Vec<String>,
    pub enable_auth: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Auto-detect database type from connection string
        let database_type = if database_url.starts_with("postgresql://")
            || database_url.starts_with("postgres://")
        {
            DatabaseType::PostgreSQL
        } else if database_url.starts_with("mssql://") || database_url.starts_with("sqlserver://") {
            DatabaseType::Mssql
        } else {
            // Fallback: check DATABASE_TYPE env var
            DatabaseType::parse(
                &env::var("DATABASE_TYPE").unwrap_or_else(|_| "postgresql".to_string()),
            )?
        };

        // Parse CORS allowed origins
        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173,http://localhost:8765".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(Config {
            database_type,
            database_url,
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8765".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            app_version: env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set for authentication"),
            cors_allowed_origins,
            enable_auth: env::var("ENABLE_AUTH")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
        })
    }
}
