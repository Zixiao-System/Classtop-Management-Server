//! Error types for mssql-driver

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    Tls(String),

    #[error("SQL Server error (code {code}): {message}")]
    ServerError {
        code: i32,
        message: String,
        line: i32,
        state: i8,
    },

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Type conversion error: {0}")]
    TypeConversion(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Connection pool exhausted")]
    PoolExhausted,

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    pub fn is_connection_error(&self) -> bool {
        matches!(self, Error::ConnectionFailed(_) | Error::Io(_))
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Error::AuthenticationFailed(_))
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self, Error::ServerError { .. })
    }
}

// Helper for creating server errors
impl Error {
    pub fn server_error(code: i32, message: String, line: i32, state: i8) -> Self {
        Error::ServerError {
            code,
            message,
            line,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_types() {
        let err = Error::ConnectionFailed("test".to_string());
        assert!(err.is_connection_error());
        assert!(!err.is_auth_error());
    }

    #[test]
    fn test_server_error() {
        let err = Error::server_error(50000, "Custom error".to_string(), 10, 1);
        assert!(err.is_server_error());
        assert!(err.to_string().contains("50000"));
    }
}
