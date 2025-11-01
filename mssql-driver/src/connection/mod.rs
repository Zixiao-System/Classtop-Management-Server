//! Connection management module

pub mod config;
pub mod pool;

pub use config::{ConnectionConfig, ConnectionConfigBuilder};
pub use pool::ConnectionPool;

use crate::error::{Error, Result};
use crate::protocol::{
    ColMetaDataToken, EncryptionLevel, Login7Packet, PacketHeader, PacketType, PreLoginPacket,
    QueryResult, SqlBatchPacket, Token, TokenParser,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// SQL Server connection
pub struct Connection {
    stream: TcpStream,
    _config: ConnectionConfig,
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

        // Step 5: Send Login7 packet with credentials
        log::debug!("Sending Login7 packet");

        let login7 = Login7Packet::new(
            config.username.clone(),
            config.password.clone(),
            config.database.clone(),
        );

        let login7_data = login7.to_bytes()?;

        let login7_header = PacketHeader {
            packet_type: PacketType::Tds7Login,
            status: 0x01, // EOM
            length: (PacketHeader::SIZE + login7_data.len()) as u16,
            spid: 0,
            packet_id: 1,
            window: 0,
        };

        stream.write_all(&login7_header.to_bytes()).await?;
        stream.write_all(&login7_data).await?;
        stream.flush().await?;

        log::debug!("Login7 packet sent ({} bytes)", login7_header.length);

        // Step 6: Receive and parse login response (Token Stream)
        log::debug!("Waiting for Login response");

        let mut login_ack_received = false;
        let mut database_name = String::new();

        // Read response packets until we get a Done token
        loop {
            // Read packet header
            let mut header_buf = [0u8; 8];
            stream.read_exact(&mut header_buf).await?;

            let response_hdr = PacketHeader::from_bytes(&header_buf).ok_or_else(|| {
                Error::ProtocolError("Invalid Login response header".to_string())
            })?;

            log::debug!("Received response packet: {:?}", response_hdr);

            // Read packet data
            let data_len = (response_hdr.length as usize)
                .checked_sub(PacketHeader::SIZE)
                .ok_or_else(|| Error::ProtocolError("Invalid packet length".to_string()))?;

            let mut packet_data = vec![0u8; data_len];
            stream.read_exact(&mut packet_data).await?;

            // Parse tokens
            let mut parser = TokenParser::new(&packet_data);

            while parser.has_more() {
                match parser.parse_next()? {
                    Token::LoginAck(ack) => {
                        log::info!(
                            "Login successful! Server: {}, TDS version: 0x{:08X}",
                            ack.prog_name,
                            ack.tds_version
                        );
                        login_ack_received = true;
                    }
                    Token::EnvChange(env) => {
                        log::info!(
                            "Environment change: {} -> {}",
                            env.old_value,
                            env.new_value
                        );
                        if env.change_type == 1 {
                            // Database change
                            database_name = env.new_value.clone();
                        }
                    }
                    Token::Done(done) => {
                        log::debug!("Done token received (status: 0x{:04X})", done.status);
                        // Check for errors in Done status
                        if done.status & 0x02 != 0 {
                            // Error flag set
                            return Err(Error::AuthenticationFailed(
                                "Login failed (Done status indicates error)".to_string(),
                            ));
                        }
                    }
                    Token::Error(err) => {
                        log::error!(
                            "SQL Server error {}: {}",
                            err.code,
                            err.message
                        );
                        return Err(Error::server_error(
                            err.code,
                            err.message,
                            err.line_number,
                            err.state as i8,
                        ));
                    }
                    Token::Info(info) => {
                        log::info!("SQL Server info: {}", info.message);
                    }
                    Token::Unknown(token_type) => {
                        log::warn!("Unknown token type: 0x{:02X}", token_type);
                    }
                    _ => {}
                }
            }

            // Check if this is the last packet (EOM bit set)
            if response_hdr.status & 0x01 != 0 {
                break;
            }
        }

        if !login_ack_received {
            return Err(Error::AuthenticationFailed(
                "No LoginAck token received".to_string(),
            ));
        }

        log::info!(
            "Authentication complete. Database: {}",
            if database_name.is_empty() {
                &config.database
            } else {
                &database_name
            }
        );

        Ok(Self {
            stream,
            _config: config,
            is_connected: true,
            packet_id: 2,
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
    pub async fn query(&mut self, sql: &str) -> Result<QueryResult> {
        if !self.is_connected {
            return Err(Error::ConnectionFailed("Not connected".to_string()));
        }

        log::debug!("Executing query: {}", sql);

        // Step 1: Build SQL_BATCH packet
        let batch_packet = SqlBatchPacket::new(sql);
        let batch_data = batch_packet.to_bytes()?;

        // Step 2: Send packet to server
        let packet_len = (PacketHeader::SIZE + batch_data.len()) as u16;
        let header = PacketHeader {
            packet_type: PacketType::SqlBatch,
            status: 0x01, // EOM (End of Message)
            length: packet_len,
            spid: 0,
            packet_id: self.packet_id,
            window: 0,
        };

        self.packet_id = self.packet_id.wrapping_add(1);

        let header_bytes = header.to_bytes();
        self.stream.write_all(&header_bytes).await?;
        self.stream.write_all(&batch_data).await?;
        self.stream.flush().await?;

        log::debug!("SQL Batch packet sent ({} bytes)", packet_len);

        // Step 3: Receive and parse token stream
        let mut column_metadata: Option<ColMetaDataToken> = None;
        let mut rows = Vec::new();
        let mut rows_affected = 0i64;

        loop {
            // Read packet header
            let mut header_buf = [0u8; PacketHeader::SIZE];
            self.stream.read_exact(&mut header_buf).await?;

            let response_hdr = PacketHeader::from_bytes(&header_buf)
                .ok_or_else(|| Error::ProtocolError("Invalid packet header".to_string()))?;

            log::debug!("Received response packet: {:?}", response_hdr);

            // Read packet data
            let data_len = response_hdr.length as usize - PacketHeader::SIZE;
            let mut packet_data = vec![0u8; data_len];
            self.stream.read_exact(&mut packet_data).await?;

            // Parse tokens - use with_metadata if we have column info
            let mut parser = if let Some(ref meta) = column_metadata {
                TokenParser::with_metadata(&packet_data, meta.columns.clone())
            } else {
                TokenParser::new(&packet_data)
            };

            while parser.has_more() {
                match parser.parse_next()? {
                    Token::ColMetaData(meta) => {
                        log::debug!("Received column metadata: {} columns", meta.columns.len());
                        column_metadata = Some(meta);
                    }
                    Token::Row(row_values) => {
                        log::debug!("Received row with {} values", row_values.len());
                        rows.push(row_values);
                    }
                    Token::Done(done) => {
                        log::debug!("Done token: row_count={}", done.row_count);
                        rows_affected = done.row_count;

                        // Check for errors in done status
                        if done.status & 0x02 != 0 {
                            // Error bit set
                            log::warn!("Done token indicates error");
                        }
                    }
                    Token::Error(err) => {
                        log::error!("SQL Server error {}: {}", err.code, err.message);
                        return Err(Error::server_error(
                            err.code,
                            err.message,
                            err.line_number,
                            err.state as i8,
                        ));
                    }
                    Token::Info(info) => {
                        log::info!("SQL Server info: {}", info.message);
                    }
                    Token::EnvChange(env) => {
                        log::debug!("Environment change: {} -> {}", env.old_value, env.new_value);
                    }
                    _ => {}
                }
            }

            // Check if this is the last packet (EOM bit set)
            if response_hdr.status & 0x01 != 0 {
                break;
            }
        }

        log::info!(
            "Query executed successfully: {} rows, {} affected",
            rows.len(),
            rows_affected
        );

        Ok(QueryResult {
            columns: column_metadata.map(|m| m.columns).unwrap_or_default(),
            rows,
            rows_affected: rows_affected as usize,
        })
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

    /// Begin a transaction
    pub async fn begin_transaction(&mut self) -> Result<()> {
        log::debug!("Beginning transaction");
        self.query("BEGIN TRANSACTION").await?;
        Ok(())
    }

    /// Commit the current transaction
    pub async fn commit(&mut self) -> Result<()> {
        log::debug!("Committing transaction");
        self.query("COMMIT").await?;
        Ok(())
    }

    /// Rollback the current transaction
    pub async fn rollback(&mut self) -> Result<()> {
        log::debug!("Rolling back transaction");
        self.query("ROLLBACK").await?;
        Ok(())
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
