use anyhow::Result;
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    PostgreSQL,
    MSSQL,
}

impl DatabaseType {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "postgresql" | "postgres" => Ok(DatabaseType::PostgreSQL),
            "mssql" | "sqlserver" => Ok(DatabaseType::MSSQL),
            _ => Err(anyhow::anyhow!("Unsupported database type: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::PostgreSQL => "PostgreSQL",
            DatabaseType::MSSQL => "SQL Server",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_type: DatabaseType,
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub app_version: String,
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
            DatabaseType::MSSQL
        } else {
            // Fallback: check DATABASE_TYPE env var
            DatabaseType::from_str(
                &env::var("DATABASE_TYPE").unwrap_or_else(|_| "postgresql".to_string()),
            )?
        };

        Ok(Config {
            database_type,
            database_url,
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8765".to_string())
                .parse()
                .expect("PORT must be a valid u16"),
            app_version: env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
        })
    }
}
