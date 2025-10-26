//! TDS (Tabular Data Stream) protocol implementation

pub mod packets;
pub mod tokens;

// Re-export commonly used types
pub use packets::{
    EncryptionLevel, Login7Packet, PacketHeader, PacketType, PreLoginPacket, PreLoginResponse,
    SqlBatchPacket,
};
pub use tokens::{ColMetaDataToken, ColumnData, EnvChangeToken, LoginAckToken, Token, TokenParser};

/// Query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<ColumnData>,
    pub rows: Vec<Vec<u8>>, // Raw row data
    pub rows_affected: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_result_creation() {
        let result = QueryResult {
            columns: vec![],
            rows: vec![],
            rows_affected: 0,
        };

        assert_eq!(result.rows.len(), 0);
        assert_eq!(result.rows_affected, 0);
    }
}
