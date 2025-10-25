//! TDS (Tabular Data Stream) protocol implementation

pub mod packets;
pub mod tokens;

// Re-export commonly used types
pub use packets::{
    EncryptionLevel, Login7Packet, PacketHeader, PacketType, PreLoginPacket, PreLoginResponse,
};

use crate::types::Value;

/// Query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Row>,
    pub rows_affected: i64,
}

/// Column metadata
#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub sql_type: String,
    pub nullable: bool,
}

/// Row data
#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<Value>,
}

impl Row {
    pub fn get<T>(&self, _index: usize) -> Option<T>
    where
        T: TryFrom<Value>,
    {
        // TODO: Implement value extraction
        None
    }

    pub fn get_by_name<T>(&self, _name: &str) -> Option<T>
    where
        T: TryFrom<Value>,
    {
        // TODO: Implement value extraction by column name
        None
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
    }
}
