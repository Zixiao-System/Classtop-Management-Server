//! TDS (Tabular Data Stream) protocol implementation

pub mod packets;
pub mod tokens;

// Re-export commonly used types
pub use packets::{
    EncryptionLevel, Login7Packet, PacketHeader, PacketType, PreLoginPacket, PreLoginResponse,
    SqlBatchPacket,
};
pub use tokens::{ColMetaDataToken, ColumnData, EnvChangeToken, LoginAckToken, Token, TokenParser};

use crate::types::Value;

/// Query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<ColumnData>,
    pub rows: Vec<Vec<Value>>,
    pub rows_affected: usize,
}

impl QueryResult {
    /// Get value at specific row and column
    pub fn get(&self, row: usize, col: usize) -> Option<&Value> {
        self.rows.get(row)?.get(col)
    }

    /// Get value by column name
    pub fn get_by_name(&self, row: usize, col_name: &str) -> Option<&Value> {
        let col_index = self
            .columns
            .iter()
            .position(|c| c.name == col_name)?;
        self.get(row, col_index)
    }
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
