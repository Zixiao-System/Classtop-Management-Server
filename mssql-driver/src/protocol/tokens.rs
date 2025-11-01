//! TDS token types and parsing

use crate::error::{Error, Result};
use crate::utils::encoding::decode_ucs2_le;

/// TDS token type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    // Result set tokens
    ColMetaData = 0x81,
    Row = 0xD1,
    NbcRow = 0xD2,
    Done = 0xFD,
    DoneProc = 0xFE,
    DoneInProc = 0xFF,

    // Error/Info tokens
    Error = 0xAA,
    Info = 0xAB,

    // Login tokens
    LoginAck = 0xAD,
    EnvChange = 0xE3,

    // Other
    ReturnStatus = 0x79,
    ReturnValue = 0xAC,
}

impl TokenType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x81 => Some(TokenType::ColMetaData),
            0xD1 => Some(TokenType::Row),
            0xD2 => Some(TokenType::NbcRow),
            0xFD => Some(TokenType::Done),
            0xFE => Some(TokenType::DoneProc),
            0xFF => Some(TokenType::DoneInProc),
            0xAA => Some(TokenType::Error),
            0xAB => Some(TokenType::Info),
            0xAD => Some(TokenType::LoginAck),
            0xE3 => Some(TokenType::EnvChange),
            0x79 => Some(TokenType::ReturnStatus),
            0xAC => Some(TokenType::ReturnValue),
            _ => None,
        }
    }
}

/// Parsed token
#[derive(Debug, Clone)]
pub enum Token {
    ColMetaData(ColMetaDataToken),
    Row(Vec<crate::types::Value>),
    Done(DoneToken),
    Error(ErrorToken),
    Info(InfoToken),
    LoginAck(LoginAckToken),
    EnvChange(EnvChangeToken),
    Unknown(u8),
}

/// TDS data type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TdsDataType {
    // Null
    Null = 0x1F,

    // Integer types
    TinyInt = 0x30,
    Bit = 0x32,
    SmallInt = 0x34,
    Int = 0x38,
    BigInt = 0x7F,

    // Decimal types
    Decimal = 0x37,
    Numeric = 0x3F,
    Float = 0x3E,
    Real = 0x3B,
    Money = 0x3C,
    SmallMoney = 0x7A,

    // String types
    Char = 0x2F,
    VarChar = 0x27,
    Text = 0x23,
    NChar = 0xEF,
    NVarChar = 0xE7,
    NText = 0x63,

    // Binary types
    Binary = 0x2D,
    VarBinary = 0x25,
    Image = 0x22,

    // Date/Time types
    DateTime = 0x3D,
    SmallDateTime = 0x3A,
    Date = 0x28,
    Time = 0x29,
    DateTime2 = 0x2A,
    DateTimeOffset = 0x2B,

    // Other types
    UniqueIdentifier = 0x24,
    Variant = 0x62,
    Xml = 0xF1,
}

impl TdsDataType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x1F => Some(TdsDataType::Null),
            0x30 => Some(TdsDataType::TinyInt),
            0x32 => Some(TdsDataType::Bit),
            0x34 => Some(TdsDataType::SmallInt),
            0x38 => Some(TdsDataType::Int),
            0x7F => Some(TdsDataType::BigInt),
            0x37 => Some(TdsDataType::Decimal),
            0x3F => Some(TdsDataType::Numeric),
            0x3E => Some(TdsDataType::Float),
            0x3B => Some(TdsDataType::Real),
            0x3C => Some(TdsDataType::Money),
            0x7A => Some(TdsDataType::SmallMoney),
            0x2F => Some(TdsDataType::Char),
            0x27 => Some(TdsDataType::VarChar),
            0x23 => Some(TdsDataType::Text),
            0xEF => Some(TdsDataType::NChar),
            0xE7 => Some(TdsDataType::NVarChar),
            0x63 => Some(TdsDataType::NText),
            0x2D => Some(TdsDataType::Binary),
            0x25 => Some(TdsDataType::VarBinary),
            0x22 => Some(TdsDataType::Image),
            0x3D => Some(TdsDataType::DateTime),
            0x3A => Some(TdsDataType::SmallDateTime),
            0x28 => Some(TdsDataType::Date),
            0x29 => Some(TdsDataType::Time),
            0x2A => Some(TdsDataType::DateTime2),
            0x2B => Some(TdsDataType::DateTimeOffset),
            0x24 => Some(TdsDataType::UniqueIdentifier),
            0x62 => Some(TdsDataType::Variant),
            0xF1 => Some(TdsDataType::Xml),
            _ => None,
        }
    }
}

/// Column metadata
#[derive(Debug, Clone)]
pub struct ColumnData {
    pub user_type: u32,
    pub flags: u16,
    pub data_type: TdsDataType,
    pub type_info: TypeInfo,
    pub name: String,
}

/// Type-specific information
#[derive(Debug, Clone)]
pub enum TypeInfo {
    FixedLen,
    VarLen { max_length: usize },
    Decimal { precision: u8, scale: u8 },
    DateTime { scale: u8 },
}

#[derive(Debug, Clone)]
pub struct ColMetaDataToken {
    pub columns: Vec<ColumnData>,
}

#[derive(Debug, Clone)]
pub struct DoneToken {
    pub status: u16,
    pub cur_cmd: u16,
    pub row_count: i64,
}

#[derive(Debug, Clone)]
pub struct ErrorToken {
    pub code: i32,
    pub state: u8,
    pub severity: u8,
    pub message: String,
    pub server_name: String,
    pub proc_name: String,
    pub line_number: i32,
}

#[derive(Debug, Clone)]
pub struct InfoToken {
    pub code: i32,
    pub state: u8,
    pub severity: u8,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct LoginAckToken {
    pub interface: u8,
    pub tds_version: u32,
    pub prog_name: String,
    pub version: u32,
}

#[derive(Debug, Clone)]
pub struct EnvChangeToken {
    pub change_type: u8,
    pub new_value: String,
    pub old_value: String,
}

impl EnvChangeToken {
    pub fn change_type_name(&self) -> &'static str {
        match self.change_type {
            1 => "Database",
            2 => "Language",
            3 => "Character Set",
            4 => "Packet Size",
            5 => "Unicode Data Sorting Local Id",
            6 => "Unicode Data Sorting Comparison Flags",
            7 => "SQL Collation",
            8 => "Begin Transaction",
            9 => "Commit Transaction",
            10 => "Rollback Transaction",
            13 => "Database Mirroring Partner",
            15 => "Promote Transaction",
            16 => "Transaction Manager Address",
            17 => "Transaction Ended",
            18 => "RESETCONNECTION/RESETCONNECTIONSKIPTRAN Completion",
            19 => "Sends Back Name of User Instance Started",
            20 => "Sends Routing Information",
            _ => "Unknown",
        }
    }
}

/// Token parser
pub struct TokenParser<'a> {
    data: &'a [u8],
    pos: usize,
    column_metadata: Option<Vec<ColumnData>>,
}

impl<'a> TokenParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            pos: 0,
            column_metadata: None,
        }
    }

    pub fn with_metadata(data: &'a [u8], metadata: Vec<ColumnData>) -> Self {
        Self {
            data,
            pos: 0,
            column_metadata: Some(metadata),
        }
    }

    pub fn has_more(&self) -> bool {
        self.pos < self.data.len()
    }

    pub fn parse_next(&mut self) -> Result<Token> {
        if !self.has_more() {
            return Err(Error::ProtocolError("No more tokens".to_string()));
        }

        let token_type = self.read_u8()?;

        match TokenType::from_u8(token_type) {
            Some(TokenType::ColMetaData) => self.parse_colmetadata(),
            Some(TokenType::Row) => self.parse_row(),
            Some(TokenType::LoginAck) => self.parse_loginack(),
            Some(TokenType::EnvChange) => self.parse_envchange(),
            Some(TokenType::Done) | Some(TokenType::DoneProc) | Some(TokenType::DoneInProc) => {
                self.parse_done()
            }
            Some(TokenType::Error) => self.parse_error(),
            Some(TokenType::Info) => self.parse_info(),
            _ => {
                log::warn!("Unknown token type: 0x{:02X}", token_type);
                Ok(Token::Unknown(token_type))
            }
        }
    }

    fn parse_loginack(&mut self) -> Result<Token> {
        let length = self.read_u16_le()?;
        let start_pos = self.pos;

        let interface = self.read_u8()?;
        let tds_version = self.read_u32_be()?;

        // Program name (B_VARCHAR)
        let prog_name_len = self.read_u8()? as usize;
        let prog_name_bytes = self.read_bytes(prog_name_len * 2)?; // UCS-2
        let prog_name = decode_ucs2_le(prog_name_bytes)?;

        // Server version
        let version = self.read_u32_be()?;

        // Verify we read the correct amount
        let bytes_read = self.pos - start_pos;
        if bytes_read != length as usize {
            log::warn!(
                "LoginAck length mismatch: expected {}, read {}",
                length,
                bytes_read
            );
        }

        Ok(Token::LoginAck(LoginAckToken {
            interface,
            tds_version,
            prog_name,
            version,
        }))
    }

    fn parse_envchange(&mut self) -> Result<Token> {
        let length = self.read_u16_le()?;
        let start_pos = self.pos;

        let change_type = self.read_u8()?;

        // New value (B_VARCHAR)
        let new_value = self.read_b_varchar()?;

        // Old value (B_VARCHAR)
        let old_value = self.read_b_varchar()?;

        // Verify length
        let bytes_read = self.pos - start_pos;
        if bytes_read != length as usize {
            log::warn!(
                "EnvChange length mismatch: expected {}, read {}",
                length,
                bytes_read
            );
        }

        Ok(Token::EnvChange(EnvChangeToken {
            change_type,
            new_value,
            old_value,
        }))
    }

    fn parse_done(&mut self) -> Result<Token> {
        let status = self.read_u16_le()?;
        let cur_cmd = self.read_u16_le()?;
        let row_count = self.read_i64_le()?;

        Ok(Token::Done(DoneToken {
            status,
            cur_cmd,
            row_count,
        }))
    }

    fn parse_error(&mut self) -> Result<Token> {
        let _length = self.read_u16_le()?;
        let _start_pos = self.pos;

        let code = self.read_i32_le()?;
        let state = self.read_u8()?;
        let severity = self.read_u8()?;

        // Message (US_VARCHAR)
        let message = self.read_us_varchar()?;

        // Server name (B_VARCHAR)
        let server_name = self.read_b_varchar()?;

        // Proc name (B_VARCHAR)
        let proc_name = self.read_b_varchar()?;

        // Line number
        let line_number = self.read_i32_le()?;

        Ok(Token::Error(ErrorToken {
            code,
            state,
            severity,
            message,
            server_name,
            proc_name,
            line_number,
        }))
    }

    fn parse_info(&mut self) -> Result<Token> {
        let _length = self.read_u16_le()?;
        let _start_pos = self.pos;

        let code = self.read_i32_le()?;
        let state = self.read_u8()?;
        let severity = self.read_u8()?;

        // Message (US_VARCHAR)
        let message = self.read_us_varchar()?;

        Ok(Token::Info(InfoToken {
            code,
            state,
            severity,
            message,
        }))
    }

    fn parse_colmetadata(&mut self) -> Result<Token> {
        let column_count = self.read_u16_le()?;

        // 0xFFFF means no metadata (for example, in UPDATE/INSERT statements)
        if column_count == 0xFFFF {
            return Ok(Token::ColMetaData(ColMetaDataToken {
                columns: Vec::new(),
            }));
        }

        let mut columns = Vec::new();

        for _ in 0..column_count {
            // User type (4 bytes)
            let user_type_bytes = self.read_bytes(4)?;
            let user_type = u32::from_le_bytes([
                user_type_bytes[0],
                user_type_bytes[1],
                user_type_bytes[2],
                user_type_bytes[3],
            ]);

            // Flags (2 bytes)
            let flags = self.read_u16_le()?;

            // Data type (1 byte)
            let data_type_byte = self.read_u8()?;
            let data_type = TdsDataType::from_u8(data_type_byte)
                .unwrap_or(TdsDataType::Null);

            // Type info (depends on data type)
            let type_info = self.parse_type_info(data_type)?;

            // Column name (B_VARCHAR)
            let name = self.read_b_varchar()?;

            columns.push(ColumnData {
                user_type,
                flags,
                data_type,
                type_info,
                name,
            });
        }

        Ok(Token::ColMetaData(ColMetaDataToken { columns }))
    }

    fn parse_type_info(&mut self, data_type: TdsDataType) -> Result<TypeInfo> {
        match data_type {
            // Fixed-length types
            TdsDataType::Bit
            | TdsDataType::TinyInt
            | TdsDataType::SmallInt
            | TdsDataType::Int
            | TdsDataType::BigInt
            | TdsDataType::Float
            | TdsDataType::Real
            | TdsDataType::Money
            | TdsDataType::SmallMoney
            | TdsDataType::DateTime
            | TdsDataType::SmallDateTime
            | TdsDataType::UniqueIdentifier => Ok(TypeInfo::FixedLen),

            // Variable-length types
            TdsDataType::VarChar | TdsDataType::NVarChar | TdsDataType::VarBinary => {
                let max_len = self.read_u16_le()? as usize;
                Ok(TypeInfo::VarLen {
                    max_length: max_len,
                })
            }

            // Decimal/Numeric types
            TdsDataType::Decimal | TdsDataType::Numeric => {
                let _len = self.read_u8()?; // Length (always 17)
                let precision = self.read_u8()?;
                let scale = self.read_u8()?;
                Ok(TypeInfo::Decimal { precision, scale })
            }

            // Date/Time types with scale
            TdsDataType::Time | TdsDataType::DateTime2 | TdsDataType::DateTimeOffset => {
                let scale = self.read_u8()?;
                Ok(TypeInfo::DateTime { scale })
            }

            TdsDataType::Date => Ok(TypeInfo::FixedLen),

            // Other types - just consume the length byte if present
            _ => Ok(TypeInfo::FixedLen),
        }
    }

    fn parse_row(&mut self) -> Result<Token> {
        use crate::types::Value;

        // Clone columns to avoid borrow issues
        let columns = self
            .column_metadata
            .as_ref()
            .ok_or_else(|| {
                Error::ProtocolError("No column metadata available for row parsing".to_string())
            })?
            .clone();

        let mut values = Vec::new();

        for column in &columns {
            let value = match column.data_type {
                // Null
                TdsDataType::Null => Value::Null,

                // Boolean
                TdsDataType::Bit => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        let byte = self.read_u8()?;
                        Value::Bit(byte != 0)
                    }
                }

                // Integers
                TdsDataType::TinyInt => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        Value::TinyInt(self.read_u8()?)
                    }
                }

                TdsDataType::SmallInt => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        Value::SmallInt(self.read_i16_le()?)
                    }
                }

                TdsDataType::Int => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        Value::Int(self.read_i32_le()?)
                    }
                }

                TdsDataType::BigInt => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        Value::BigInt(self.read_i64_le()?)
                    }
                }

                // Floats
                TdsDataType::Real => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        let bytes = self.read_bytes(4)?;
                        Value::Real(f32::from_le_bytes([
                            bytes[0], bytes[1], bytes[2], bytes[3],
                        ]))
                    }
                }

                TdsDataType::Float => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        let bytes = self.read_bytes(8)?;
                        Value::Float(f64::from_le_bytes([
                            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                            bytes[7],
                        ]))
                    }
                }

                // Strings - variable length
                TdsDataType::NVarChar | TdsDataType::NChar => {
                    let len = self.read_u16_le()? as usize;
                    if len == 0xFFFF {
                        // Null
                        Value::Null
                    } else if len == 0 {
                        Value::NVarChar(String::new())
                    } else {
                        let bytes = self.read_bytes(len)?;
                        let text = decode_ucs2_le(bytes)?;
                        Value::NVarChar(text)
                    }
                }

                TdsDataType::VarChar | TdsDataType::Char => {
                    let len = self.read_u16_le()? as usize;
                    if len == 0xFFFF {
                        Value::Null
                    } else if len == 0 {
                        Value::VarChar(String::new())
                    } else {
                        let bytes = self.read_bytes(len)?;
                        // Assuming UTF-8 for VARCHAR (should use collation)
                        Value::VarChar(String::from_utf8_lossy(bytes).to_string())
                    }
                }

                // Binary
                TdsDataType::VarBinary | TdsDataType::Binary => {
                    let len = self.read_u16_le()? as usize;
                    if len == 0xFFFF {
                        Value::Null
                    } else {
                        let bytes = self.read_bytes(len)?;
                        Value::VarBinary(bytes.to_vec())
                    }
                }

                // UniqueIdentifier (GUID)
                TdsDataType::UniqueIdentifier => {
                    let len = self.read_u8()?;
                    if len == 0 {
                        Value::Null
                    } else {
                        let bytes = self.read_bytes(16)?;
                        let uuid = uuid::Uuid::from_bytes_le([
                            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                            bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12],
                            bytes[13], bytes[14], bytes[15],
                        ]);
                        Value::UniqueIdentifier(uuid)
                    }
                }

                // For unsupported types, skip and return Null
                _ => {
                    log::warn!("Unsupported data type in row: {:?}", column.data_type);
                    // Try to skip the value (assuming 1-byte length prefix)
                    let len = self.read_u8().unwrap_or(0) as usize;
                    if len > 0 && len < 0xFF {
                        let _ = self.read_bytes(len);
                    }
                    Value::Null
                }
            };

            values.push(value);
        }

        Ok(Token::Row(values))
    }

    // Helper methods for reading data
    fn read_u8(&mut self) -> Result<u8> {
        if self.pos >= self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let value = self.data[self.pos];
        self.pos += 1;
        Ok(value)
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        if self.pos + 2 > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let value = u16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    fn read_i16_le(&mut self) -> Result<i16> {
        if self.pos + 2 > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let value = i16::from_le_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    fn read_i32_le(&mut self) -> Result<i32> {
        if self.pos + 4 > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let value = i32::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    fn read_u32_be(&mut self) -> Result<u32> {
        if self.pos + 4 > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let value = u32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    fn read_i64_le(&mut self) -> Result<i64> {
        if self.pos + 8 > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let value = i64::from_le_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        Ok(value)
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.pos + len > self.data.len() {
            return Err(Error::UnexpectedEof);
        }
        let bytes = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(bytes)
    }

    fn read_b_varchar(&mut self) -> Result<String> {
        let len = self.read_u8()? as usize;
        if len == 0 {
            return Ok(String::new());
        }
        let bytes = self.read_bytes(len * 2)?; // UCS-2
        decode_ucs2_le(bytes)
    }

    fn read_us_varchar(&mut self) -> Result<String> {
        let len = self.read_u16_le()? as usize;
        if len == 0 {
            return Ok(String::new());
        }
        let bytes = self.read_bytes(len * 2)?; // UCS-2
        decode_ucs2_le(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_from_u8() {
        assert_eq!(TokenType::from_u8(0xAD), Some(TokenType::LoginAck));
        assert_eq!(TokenType::from_u8(0xE3), Some(TokenType::EnvChange));
        assert_eq!(TokenType::from_u8(0xFD), Some(TokenType::Done));
        assert_eq!(TokenType::from_u8(0xFF), Some(TokenType::DoneInProc));
        assert_eq!(TokenType::from_u8(0x00), None); // Invalid token type
    }

    #[test]
    fn test_envchange_type_name() {
        let token = EnvChangeToken {
            change_type: 1,
            new_value: "testdb".to_string(),
            old_value: "master".to_string(),
        };
        assert_eq!(token.change_type_name(), "Database");
    }
}

