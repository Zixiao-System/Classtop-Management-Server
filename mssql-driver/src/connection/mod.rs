//! Connection management module

pub mod config;
pub mod pool;

pub use config::{ConnectionConfig, ConnectionConfigBuilder};
pub use pool::ConnectionPool;

use crate::error::{Error, Result};
use crate::protocol::{EncryptionLevel, PacketHeader, PacketType, PreLoginPacket, QueryResult};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// SQL Server connection
pub struct Connection {
    stream: TcpStream,
    config: ConnectionConfig,
    is_connected: bool,
    packet_id: u8,
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
        let mut stream = tokio::time::timeout(config.connect_timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| Error::Timeout(format!("Connection to {} timed out", addr)))?
            .map_err(|e| {
                Error::ConnectionFailed(format!("Failed to connect to {}: {}", addr, e))
            })?;

        log::info!("TCP connection established");

        // Step 2: Send Pre-Login packet
        log::debug!("Sending Pre-Login packet");

        let prelogin = PreLoginPacket::new().with_encryption(if config.encrypt {
            EncryptionLevel::EncryptOn
        } else {
            EncryptionLevel::EncryptNotSup
        });

        let prelogin_data = prelogin.to_bytes()?;

        // Create packet header
        let header = PacketHeader {
            packet_type: PacketType::PreLogin,
            status: 0x01, // EOM (End of Message)
            length: (PacketHeader::SIZE + prelogin_data.len()) as u16,
            spid: 0,
            packet_id: 0,
            window: 0,
        };

        // Send header + data
        stream.write_all(&header.to_bytes()).await?;
        stream.write_all(&prelogin_data).await?;
        stream.flush().await?;

        log::debug!("Pre-Login packet sent ({} bytes)", header.length);

        // Step 3: Receive Pre-Login response
        log::debug!("Waiting for Pre-Login response");

        let mut response_header = [0u8; 8];
        stream.read_exact(&mut response_header).await?;

        let response_hdr = PacketHeader::from_bytes(&response_header)
            .ok_or_else(|| Error::ProtocolError("Invalid Pre-Login response header".to_string()))?;

        log::debug!("Received Pre-Login response header: {:?}", response_hdr);

        // Read response data
        let data_len = (response_hdr.length as usize)
            .checked_sub(PacketHeader::SIZE)
            .ok_or_else(|| Error::ProtocolError("Invalid response length".to_string()))?;

        let mut response_data = vec![0u8; data_len];
        stream.read_exact(&mut response_data).await?;

        // Parse Pre-Login response
        let prelogin_response = PreLoginPacket::from_bytes(&response_data)?;

        log::info!(
            "Pre-Login complete. Encryption: {:?}",
            prelogin_response.encryption
        );

        // Step 4: Negotiate TLS if required
        if prelogin_response.encryption_required() {
            log::warn!("TLS encryption required but not yet implemented");
            return Err(Error::Tls(
                "TLS encryption not yet implemented".to_string(),
            ));
        }

        // TODO: Step 5: Send Login7 packet with credentials
        // TODO: Step 6: Receive login response

        log::info!("Connection established (Pre-Login complete, Login7 TODO)");

        Ok(Self {
            stream,
            config,
            is_connected: true,
            packet_id: 1,
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
    pub async fn execute(
        &mut self,
        _sql: &str,
        _params: Vec<crate::types::Parameter>,
    ) -> Result<QueryResult> {
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
