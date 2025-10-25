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
    Row(Vec<u8>),
    Done(DoneToken),
    Error(ErrorToken),
    Info(InfoToken),
    LoginAck(LoginAckToken),
    EnvChange(EnvChangeToken),
    Unknown(u8),
}

#[derive(Debug, Clone)]
pub struct ColMetaDataToken {
    pub column_count: u16,
    // TODO: Add column metadata fields
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
}

impl<'a> TokenParser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
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
        let length = self.read_u16_le()?;
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
        let length = self.read_u16_le()?;
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

