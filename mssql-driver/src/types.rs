//! Type system for Rust ↔ SQL Server conversions

use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

/// SQL Server value types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bit(bool),
    TinyInt(u8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Real(f32),
    Float(f64),
    Decimal(String), // TODO: Use proper decimal type
    VarChar(String),
    NVarChar(String),
    DateTime(NaiveDateTime),
    DateTime2(NaiveDateTime),
    DateTimeOffset(DateTime<Utc>),
    UniqueIdentifier(Uuid),
    Binary(Vec<u8>),
    VarBinary(Vec<u8>),
}

/// Parameter for parameterized queries
#[derive(Debug, Clone)]
pub enum Parameter {
    Null,
    Bit(bool),
    Int(i32),
    BigInt(i64),
    Float(f64),
    NVarChar(String),
    DateTime2(NaiveDateTime),
    UniqueIdentifier(Uuid),
    VarBinary(Vec<u8>),
}

impl Parameter {
    /// Get SQL type name for this parameter
    pub fn sql_type(&self) -> &'static str {
        match self {
            Parameter::Null => "NULL",
            Parameter::Bit(_) => "BIT",
            Parameter::Int(_) => "INT",
            Parameter::BigInt(_) => "BIGINT",
            Parameter::Float(_) => "FLOAT",
            Parameter::NVarChar(_) => "NVARCHAR(MAX)",
            Parameter::DateTime2(_) => "DATETIME2",
            Parameter::UniqueIdentifier(_) => "UNIQUEIDENTIFIER",
            Parameter::VarBinary(_) => "VARBINARY(MAX)",
        }
    }

    /// Encode parameter value to bytes (TDS format)
    pub fn encode(&self) -> Vec<u8> {
        // TODO: Implement proper TDS encoding
        match self {
            Parameter::Null => vec![],
            Parameter::Bit(v) => vec![if *v { 1 } else { 0 }],
            Parameter::Int(v) => v.to_le_bytes().to_vec(),
            Parameter::BigInt(v) => v.to_le_bytes().to_vec(),
            Parameter::Float(v) => v.to_le_bytes().to_vec(),
            Parameter::NVarChar(s) => {
                // UCS-2 LE encoding
                let utf16: Vec<u8> = s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect();
                utf16
            }
            Parameter::UniqueIdentifier(uuid) => uuid.as_bytes().to_vec(),
            Parameter::VarBinary(bytes) => bytes.clone(),
            _ => vec![],
        }
    }
}

// Conversion implementations
impl From<bool> for Parameter {
    fn from(v: bool) -> Self {
        Parameter::Bit(v)
    }
}

impl From<i32> for Parameter {
    fn from(v: i32) -> Self {
        Parameter::Int(v)
    }
}

impl From<i64> for Parameter {
    fn from(v: i64) -> Self {
        Parameter::BigInt(v)
    }
}

impl From<f64> for Parameter {
    fn from(v: f64) -> Self {
        Parameter::Float(v)
    }
}

impl From<String> for Parameter {
    fn from(v: String) -> Self {
        Parameter::NVarChar(v)
    }
}

impl From<&str> for Parameter {
    fn from(v: &str) -> Self {
        Parameter::NVarChar(v.to_string())
    }
}

impl From<Uuid> for Parameter {
    fn from(v: Uuid) -> Self {
        Parameter::UniqueIdentifier(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_conversions() {
        let p1: Parameter = 42.into();
        assert!(matches!(p1, Parameter::Int(42)));

        let p2: Parameter = "hello".into();
        assert!(matches!(p2, Parameter::NVarChar(_)));

        let p3: Parameter = true.into();
        assert!(matches!(p3, Parameter::Bit(true)));
    }

    #[test]
    fn test_parameter_sql_type() {
        assert_eq!(Parameter::Int(42).sql_type(), "INT");
        assert_eq!(
            Parameter::NVarChar("test".to_string()).sql_type(),
            "NVARCHAR(MAX)"
        );
    }
}
