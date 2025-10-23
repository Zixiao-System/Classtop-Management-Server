//! Connection management module

pub mod config;
pub mod pool;

pub use config::{ConnectionConfig, ConnectionConfigBuilder};
pub use pool::ConnectionPool;

use crate::error::{Error, Result};
use crate::protocol::QueryResult;
use tokio::net::TcpStream;

/// SQL Server connection
pub struct Connection {
    stream: TcpStream,
    config: ConnectionConfig,
    is_connected: bool,
}

impl Connection {
    /// Connect to SQL Server
    ///
    /// # Example
    ///
    /// ```no_run
    /// use mssql_driver::{Connection, ConnectionConfig};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ConnectionConfig::builder()
    ///     .host("localhost")
    ///     .port(1433)
    ///     .username("sa")
    ///     .password("Password123")
    ///     .database("master")
    ///     .build()?;
    ///
    /// let conn = Connection::connect(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(config: ConnectionConfig) -> Result<Self> {
        config.validate()?;

        log::info!("Connecting to {}:{}", config.host, config.port);

        // Step 1: Establish TCP connection
        let addr = format!("{}:{}", config.host, config.port);
        let stream = tokio::time::timeout(
            config.connect_timeout,
            TcpStream::connect(&addr)
        )
        .await
        .map_err(|_| Error::Timeout(format!("Connection to {} timed out", addr)))?
        .map_err(|e| Error::ConnectionFailed(format!("Failed to connect to {}: {}", addr, e)))?;

        log::info!("TCP connection established");

        // TODO: Implement TDS protocol handshake
        // Step 2: Send Pre-Login packet
        // Step 3: Negotiate TLS if required
        // Step 4: Send Login7 packet with credentials
        // Step 5: Receive login response

        Ok(Self {
            stream,
            config,
            is_connected: true,
        })
    }

    /// Execute a SQL query
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use mssql_driver::Connection;
    /// # async fn example(mut conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    /// let result = conn.query("SELECT 1 AS num, 'hello' AS msg").await?;
    /// println!("Rows: {}", result.rows.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query(&mut self, _sql: &str) -> Result<QueryResult> {
        if !self.is_connected {
            return Err(Error::ConnectionFailed("Not connected".to_string()));
        }

        // TODO: Implement query execution
        // Step 1: Build SQL_BATCH packet
        // Step 2: Send packet to server
        // Step 3: Receive token stream
        // Step 4: Parse result set

        log::warn!("query() not yet implemented");
        Err(Error::QueryFailed("Not implemented yet".to_string()))
    }

    /// Execute a parameterized query
    pub async fn execute(&mut self, _sql: &str, _params: Vec<crate::types::Parameter>) -> Result<QueryResult> {
        if !self.is_connected {
            return Err(Error::ConnectionFailed("Not connected".to_string()));
        }

        // TODO: Implement parameterized query execution via sp_executesql
        log::warn!("execute() not yet implemented");
        Err(Error::QueryFailed("Not implemented yet".to_string()))
    }

    /// Check if connection is alive
    pub fn is_alive(&self) -> bool {
        self.is_connected
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        if self.is_connected {
            // TODO: Send logout packet
            self.is_connected = false;
            log::info!("Connection closed");
        }
        Ok(())
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.is_connected {
            log::warn!("Connection dropped without explicit close()");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_config() {
        let config = ConnectionConfig::builder()
            .host("localhost")
            .port(1433)
            .username("sa")
            .password("test")
            .database("master")
            .build()
            .unwrap();

        assert_eq!(config.host, "localhost");
    }
}
