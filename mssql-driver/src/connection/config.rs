//! Connection configuration and management

use crate::error::{Error, Result};
use std::time::Duration;

/// Configuration for SQL Server connection
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Server hostname or IP address
    pub host: String,

    /// Server port (default: 1433)
    pub port: u16,

    /// Username for authentication
    pub username: String,

    /// Password for authentication
    pub password: String,

    /// Database name to connect to
    pub database: String,

    /// Enable TLS encryption (recommended)
    pub encrypt: bool,

    /// Trust server certificate (for self-signed certs)
    pub trust_server_certificate: bool,

    /// Connection timeout
    pub connect_timeout: Duration,

    /// Application name (sent to server)
    pub application_name: String,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 1433,
            username: String::new(),
            password: String::new(),
            database: "master".to_string(),
            encrypt: true,
            trust_server_certificate: false,
            connect_timeout: Duration::from_secs(30),
            application_name: "mssql-driver".to_string(),
        }
    }
}

impl ConnectionConfig {
    /// Create a new configuration builder
    pub fn builder() -> ConnectionConfigBuilder {
        ConnectionConfigBuilder::default()
    }

    /// Parse connection string
    ///
    /// Format: `mssql://username:password@host:port/database?options`
    ///
    /// Example: `mssql://sa:Password123@localhost:1433/mydb?encrypt=true`
    pub fn from_connection_string(conn_str: &str) -> Result<Self> {
        // TODO: Implement full connection string parsing
        // For now, return a basic implementation
        if !conn_str.starts_with("mssql://") && !conn_str.starts_with("sqlserver://") {
            return Err(Error::InvalidConfig(
                "Connection string must start with 'mssql://' or 'sqlserver://'".to_string(),
            ));
        }

        // Strip protocol
        let without_protocol = conn_str
            .strip_prefix("mssql://")
            .or_else(|| conn_str.strip_prefix("sqlserver://"))
            .ok_or_else(|| Error::InvalidConfig("Invalid connection string".to_string()))?;

        // Basic parsing (to be improved)
        let parts: Vec<&str> = without_protocol.split('@').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidConfig(
                "Invalid connection string format".to_string(),
            ));
        }

        let auth_parts: Vec<&str> = parts[0].split(':').collect();
        if auth_parts.len() != 2 {
            return Err(Error::InvalidConfig(
                "Invalid authentication format".to_string(),
            ));
        }

        let username = auth_parts[0].to_string();
        let password = auth_parts[1].to_string();

        let server_parts: Vec<&str> = parts[1].split('/').collect();
        if server_parts.is_empty() {
            return Err(Error::InvalidConfig("Invalid server format".to_string()));
        }

        let host_port: Vec<&str> = server_parts[0].split(':').collect();
        let host = host_port[0].to_string();
        let port = if host_port.len() > 1 {
            host_port[1].parse().unwrap_or(1433)
        } else {
            1433
        };

        let database = if server_parts.len() > 1 {
            server_parts[1]
                .split('?')
                .next()
                .unwrap_or("master")
                .to_string()
        } else {
            "master".to_string()
        };

        Ok(Self {
            host,
            port,
            username,
            password,
            database,
            ..Default::default()
        })
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(Error::InvalidConfig("Host cannot be empty".to_string()));
        }
        if self.username.is_empty() {
            return Err(Error::InvalidConfig("Username cannot be empty".to_string()));
        }
        if self.port == 0 {
            return Err(Error::InvalidConfig("Invalid port number".to_string()));
        }
        Ok(())
    }
}

/// Builder for ConnectionConfig
#[derive(Default)]
pub struct ConnectionConfigBuilder {
    config: ConnectionConfig,
}

impl ConnectionConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.config.username = username.into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.config.password = password.into();
        self
    }

    pub fn database(mut self, database: impl Into<String>) -> Self {
        self.config.database = database.into();
        self
    }

    pub fn encrypt(mut self, encrypt: bool) -> Self {
        self.config.encrypt = encrypt;
        self
    }

    pub fn trust_server_certificate(mut self, trust: bool) -> Self {
        self.config.trust_server_certificate = trust;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.config.connect_timeout = timeout;
        self
    }

    pub fn application_name(mut self, name: impl Into<String>) -> Self {
        self.config.application_name = name.into();
        self
    }

    pub fn build(self) -> Result<ConnectionConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = ConnectionConfig::builder()
            .host("localhost")
            .port(1433)
            .username("sa")
            .password("Password123")
            .database("testdb")
            .build()
            .unwrap();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 1433);
        assert_eq!(config.database, "testdb");
    }

    #[test]
    fn test_connection_string_parsing() {
        let conn_str = "mssql://sa:Password123@localhost:1433/testdb";
        let config = ConnectionConfig::from_connection_string(conn_str).unwrap();

        assert_eq!(config.username, "sa");
        assert_eq!(config.password, "Password123");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 1433);
        assert_eq!(config.database, "testdb");
    }

    #[test]
    fn test_config_validation() {
        let invalid_config = ConnectionConfig {
            host: String::new(),
            ..Default::default()
        };

        assert!(invalid_config.validate().is_err());
    }
}
