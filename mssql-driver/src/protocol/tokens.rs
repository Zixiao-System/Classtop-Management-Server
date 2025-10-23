//! TDS token types and parsing

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

/// Parsed token
#[derive(Debug, Clone)]
pub enum Token {
    ColMetaData(ColMetaDataToken),
    Row(Vec<u8>),
    Done(DoneToken),
    Error(ErrorToken),
    Info(InfoToken),
    LoginAck(LoginAckToken),
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
