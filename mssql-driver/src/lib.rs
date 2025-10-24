//! # mssql-driver
//!
//! A pure Rust implementation of Microsoft SQL Server TDS (Tabular Data Stream) protocol driver.
//!
//! ## Features
//!
//! - Pure Rust implementation with no C dependencies
//! - Async/await support via Tokio
//! - TLS/SSL encryption support
//! - Connection pooling
//! - Parameterized queries
//! - Transaction support
//!
//! ## Example
//!
//! ```rust,no_run
//! use mssql_driver::{Connection, ConnectionConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ConnectionConfig {
//!         host: "localhost".to_string(),
//!         port: 1433,
//!         username: "sa".to_string(),
//!         password: "YourPassword".to_string(),
//!         database: "master".to_string(),
//!         encrypt: true,
//!     };
//!
//!     let mut conn = Connection::connect(config).await?;
//!
//!     let result = conn.query("SELECT 1 AS num").await?;
//!     println!("Result: {:?}", result);
//!
//!     Ok(())
//! }
//! ```

pub mod connection;
pub mod error;
pub mod protocol;
pub mod transaction;
pub mod types;
mod utils;

// Re-exports for convenience
pub use connection::{Connection, ConnectionConfig, ConnectionPool};
pub use error::{Error, Result};
pub use protocol::QueryResult;
pub use transaction::Transaction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.0");
    }
}
