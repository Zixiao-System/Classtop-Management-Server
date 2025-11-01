//! TDS packet structures and definitions

use crate::error::{Error, Result};
use std::collections::HashMap;

/// TDS packet type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PacketType {
    SqlBatch = 0x01,
    PreTDSLogin = 0x02,
    Rpc = 0x03,
    TabularResult = 0x04,
    AttentionSignal = 0x06,
    BulkLoadData = 0x07,
    FederatedAuthToken = 0x08,
    TransactionManagerRequest = 0x0E,
    Tds7Login = 0x10,
    Sspi = 0x11,
    PreLogin = 0x12,
}

/// TDS packet header (8 bytes)
#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub packet_type: PacketType,
    pub status: u8,
    pub length: u16,
    pub spid: u16,
    pub packet_id: u8,
    pub window: u8,
}

impl PacketHeader {
    pub const SIZE: usize = 8;

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[0] = self.packet_type as u8;
        bytes[1] = self.status;
        bytes[2..4].copy_from_slice(&self.length.to_be_bytes());
        bytes[4..6].copy_from_slice(&self.spid.to_be_bytes());
        bytes[6] = self.packet_id;
        bytes[7] = self.window;
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }

        Some(Self {
            packet_type: match bytes[0] {
                0x01 => PacketType::SqlBatch,
                0x04 => PacketType::TabularResult,
                0x12 => PacketType::PreLogin,
                _ => return None,
            },
            status: bytes[1],
            length: u16::from_be_bytes([bytes[2], bytes[3]]),
            spid: u16::from_be_bytes([bytes[4], bytes[5]]),
            packet_id: bytes[6],
            window: bytes[7],
        })
    }
}

// ===================================================================
// Pre-Login Packet
// ===================================================================

/// Pre-Login option token types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PreLoginOptionToken {
    Version = 0x00,
    Encryption = 0x01,
    InstOpt = 0x02,
    ThreadId = 0x03,
    Mars = 0x04,
    TraceId = 0x05,
    FedAuthRequired = 0x06,
    NonceOpt = 0x07,
    Terminator = 0xFF,
}

/// Encryption level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EncryptionLevel {
    EncryptOff = 0x00,
    EncryptOn = 0x01,
    EncryptNotSup = 0x02,
    EncryptReq = 0x03,
    EncryptClientCert = 0x04,
}

/// Pre-Login packet
#[derive(Debug, Clone)]
pub struct PreLoginPacket {
    pub version: (u8, u8, u16, u16), // (major, minor, build, subbuild)
    pub encryption: EncryptionLevel,
    pub instance_name: Option<String>,
    pub thread_id: u32,
    pub mars: bool,
}

impl Default for PreLoginPacket {
    fn default() -> Self {
        Self {
            version: (16, 0, 0, 0), // SQL Server 2022
            encryption: EncryptionLevel::EncryptNotSup,
            instance_name: None,
            thread_id: 0,
            mars: false, // MARS (Multiple Active Result Sets)
        }
    }
}

impl PreLoginPacket {
    /// Create a new Pre-Login packet with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set encryption level
    pub fn with_encryption(mut self, encryption: EncryptionLevel) -> Self {
        self.encryption = encryption;
        self
    }

    /// Set instance name
    pub fn with_instance(mut self, instance: String) -> Self {
        self.instance_name = Some(instance);
        self
    }

    /// Enable MARS (Multiple Active Result Sets)
    pub fn with_mars(mut self, mars: bool) -> Self {
        self.mars = mars;
        self
    }

    /// Encode Pre-Login packet to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut data = Vec::new();

        // Build option offsets
        let mut options_data = Vec::new();

        // Calculate base offset (after all option headers + terminator)
        // Each option: 1 byte token + 2 bytes offset + 2 bytes length = 5 bytes
        // Terminator: 1 byte
        let mut num_options = 0;
        if true { num_options += 1; } // VERSION (always present)
        if true { num_options += 1; } // ENCRYPTION (always present)
        if self.instance_name.is_some() { num_options += 1; }
        if true { num_options += 1; } // THREADID (always present)
        if self.mars { num_options += 1; }

        let mut offset = (num_options * 5 + 1) as u16; // +1 for terminator

        // VERSION option
        data.push(PreLoginOptionToken::Version as u8);
        data.extend_from_slice(&offset.to_be_bytes());
        let version_data = self.encode_version();
        data.extend_from_slice(&(version_data.len() as u16).to_be_bytes());
        options_data.extend_from_slice(&version_data);
        offset += version_data.len() as u16;

        // ENCRYPTION option
        data.push(PreLoginOptionToken::Encryption as u8);
        data.extend_from_slice(&offset.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes()); // length = 1
        options_data.push(self.encryption as u8);
        offset += 1;

        // THREADID option
        data.push(PreLoginOptionToken::ThreadId as u8);
        data.extend_from_slice(&offset.to_be_bytes());
        data.extend_from_slice(&4u16.to_be_bytes()); // length = 4
        options_data.extend_from_slice(&self.thread_id.to_be_bytes());
        offset += 4;

        // MARS option (if enabled)
        if self.mars {
            data.push(PreLoginOptionToken::Mars as u8);
            data.extend_from_slice(&offset.to_be_bytes());
            data.extend_from_slice(&1u16.to_be_bytes()); // length = 1
            options_data.push(0x01); // MARS enabled
            offset += 1;
        }

        // INSTANCE option (if provided)
        if let Some(ref instance) = self.instance_name {
            data.push(PreLoginOptionToken::InstOpt as u8);
            data.extend_from_slice(&offset.to_be_bytes());
            let instance_bytes = instance.as_bytes();
            data.extend_from_slice(&(instance_bytes.len() as u16).to_be_bytes());
            options_data.extend_from_slice(instance_bytes);
            options_data.push(0x00); // null terminator
        }

        // Terminator
        data.push(PreLoginOptionToken::Terminator as u8);

        // Append all option data
        data.extend_from_slice(&options_data);

        Ok(data)
    }

    /// Encode version to 6 bytes (UL_VERSION format)
    fn encode_version(&self) -> Vec<u8> {
        let (major, minor, build, subbuild) = self.version;
        vec![
            major,
            minor,
            (build >> 8) as u8,
            (build & 0xFF) as u8,
            (subbuild >> 8) as u8,
            (subbuild & 0xFF) as u8,
        ]
    }

    /// Parse Pre-Login response from server
    pub fn from_bytes(bytes: &[u8]) -> Result<PreLoginResponse> {
        if bytes.is_empty() {
            return Err(Error::ProtocolError("Empty Pre-Login response".to_string()));
        }

        let mut options = HashMap::new();
        let mut pos = 0;

        // Parse option headers
        loop {
            if pos >= bytes.len() {
                return Err(Error::ProtocolError("Incomplete Pre-Login response".to_string()));
            }

            let token = bytes[pos];
            pos += 1;

            if token == PreLoginOptionToken::Terminator as u8 {
                break;
            }

            if pos + 4 > bytes.len() {
                return Err(Error::ProtocolError("Incomplete option header".to_string()));
            }

            let offset = u16::from_be_bytes([bytes[pos], bytes[pos + 1]]) as usize;
            let length = u16::from_be_bytes([bytes[pos + 2], bytes[pos + 3]]) as usize;
            pos += 4;

            if offset + length > bytes.len() {
                return Err(Error::ProtocolError("Invalid option offset/length".to_string()));
            }

            let data = &bytes[offset..offset + length];
            options.insert(token, data.to_vec());
        }

        // Parse ENCRYPTION option (required)
        let encryption = options.get(&(PreLoginOptionToken::Encryption as u8))
            .and_then(|data| data.first().copied())
            .map(|e| match e {
                0x00 => EncryptionLevel::EncryptOff,
                0x01 => EncryptionLevel::EncryptOn,
                0x02 => EncryptionLevel::EncryptNotSup,
                0x03 => EncryptionLevel::EncryptReq,
                0x04 => EncryptionLevel::EncryptClientCert,
                _ => EncryptionLevel::EncryptOff,
            })
            .ok_or_else(|| Error::ProtocolError("Missing ENCRYPTION option".to_string()))?;

        Ok(PreLoginResponse {
            encryption,
            options,
        })
    }
}

/// Pre-Login response from server
#[derive(Debug, Clone)]
pub struct PreLoginResponse {
    pub encryption: EncryptionLevel,
    pub options: HashMap<u8, Vec<u8>>,
}

impl PreLoginResponse {
    /// Check if encryption is required
    pub fn encryption_required(&self) -> bool {
        matches!(self.encryption, EncryptionLevel::EncryptOn | EncryptionLevel::EncryptReq)
    }
}

// ===================================================================
// Login7 Packet
// ===================================================================

/// Login7 packet for authentication
#[derive(Debug, Clone)]
pub struct Login7Packet {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub app_name: String,
    pub server_name: String,
    pub client_interface_name: String,
    pub language: String,
    pub database: String,
    pub client_id: [u8; 6],
}

impl Login7Packet {
    /// Create a new Login7 packet
    pub fn new(username: String, password: String, database: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        // Generate client ID (first 6 bytes of process ID + timestamp)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        let pid = std::process::id();
        let client_id = [
            (pid & 0xFF) as u8,
            ((pid >> 8) & 0xFF) as u8,
            (timestamp & 0xFF) as u8,
            ((timestamp >> 8) & 0xFF) as u8,
            ((timestamp >> 16) & 0xFF) as u8,
            ((timestamp >> 24) & 0xFF) as u8,
        ];

        Self {
            hostname: std::env::var("HOSTNAME")
                .ok()
                .unwrap_or_else(|| "unknown".to_string()),
            username,
            password,
            app_name: "mssql-driver".to_string(),
            server_name: String::new(),
            client_interface_name: "mssql-driver".to_string(),
            language: String::new(), // Use server default
            database,
            client_id,
        }
    }

    /// Encode Login7 packet to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        use crate::utils::encoding::encode_ucs2_le;

        let mut data = Vec::new();

        // Fixed header (94 bytes before variable data)
        let mut header = vec![0u8; 94];

        // Length (to be filled later)
        // header[0..4] = length (u32 LE)

        // TDS version (0x74000004 = TDS 7.4)
        header[4..8].copy_from_slice(&0x74000004u32.to_le_bytes());

        // Packet size (4096 bytes)
        header[8..12].copy_from_slice(&4096u32.to_le_bytes());

        // Client program version (1.0.0.0)
        header[12..16].copy_from_slice(&0x01000000u32.to_le_bytes());

        // Client PID
        header[16..20].copy_from_slice(&std::process::id().to_le_bytes());

        // Connection ID (0)
        header[20..24].copy_from_slice(&0u32.to_le_bytes());

        // Option flags 1
        // fByteOrder(1) | fChar(1) | fFloat(0) | fDumpLoad(1) | fUseDb(1) | fDatabase(0) | fSetLang(0)
        header[24] = 0b11110000;

        // Option flags 2
        // fLanguage(0) | fODBC(1) | fTranBoundary(0) | fCacheConnect(0) | fUserType(0) | fIntSecurity(0)
        header[25] = 0b00000011;

        // Type flags
        header[26] = 0;

        // Option flags 3
        header[27] = 0;

        // Client timezone (0)
        header[28..32].copy_from_slice(&0i32.to_le_bytes());

        // Client LCID (0x00000409 = en-US)
        header[32..36].copy_from_slice(&0x00000409u32.to_le_bytes());

        // Now encode variable-length data
        let mut var_data = Vec::new();
        let mut offset = 94u16; // Start after fixed header

        // HostName (offset 36, length 38)
        let hostname_encoded = encode_ucs2_le(&self.hostname);
        header[36..38].copy_from_slice(&offset.to_le_bytes());
        header[38..40].copy_from_slice(&(self.hostname.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&hostname_encoded);
        offset += hostname_encoded.len() as u16;

        // UserName (offset 40, length 42)
        let username_encoded = encode_ucs2_le(&self.username);
        header[40..42].copy_from_slice(&offset.to_le_bytes());
        header[42..44].copy_from_slice(&(self.username.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&username_encoded);
        offset += username_encoded.len() as u16;

        // Password (offset 44, length 46) - needs to be obfuscated
        let password_encoded = self.encode_password(&self.password);
        header[44..46].copy_from_slice(&offset.to_le_bytes());
        header[46..48].copy_from_slice(&(self.password.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&password_encoded);
        offset += password_encoded.len() as u16;

        // AppName (offset 48, length 50)
        let appname_encoded = encode_ucs2_le(&self.app_name);
        header[48..50].copy_from_slice(&offset.to_le_bytes());
        header[50..52].copy_from_slice(&(self.app_name.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&appname_encoded);
        offset += appname_encoded.len() as u16;

        // ServerName (offset 52, length 54)
        let servername_encoded = encode_ucs2_le(&self.server_name);
        header[52..54].copy_from_slice(&offset.to_le_bytes());
        header[54..56].copy_from_slice(&(self.server_name.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&servername_encoded);
        offset += servername_encoded.len() as u16;

        // Extension (offset 56, length 58) - unused
        header[56..58].copy_from_slice(&offset.to_le_bytes());
        header[58..60].copy_from_slice(&0u16.to_le_bytes());

        // InterfaceName (offset 60, length 62)
        let interface_encoded = encode_ucs2_le(&self.client_interface_name);
        header[60..62].copy_from_slice(&offset.to_le_bytes());
        header[62..64].copy_from_slice(&(self.client_interface_name.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&interface_encoded);
        offset += interface_encoded.len() as u16;

        // Language (offset 64, length 66)
        let language_encoded = encode_ucs2_le(&self.language);
        header[64..66].copy_from_slice(&offset.to_le_bytes());
        header[66..68].copy_from_slice(&(self.language.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&language_encoded);
        offset += language_encoded.len() as u16;

        // Database (offset 68, length 70)
        let database_encoded = encode_ucs2_le(&self.database);
        header[68..70].copy_from_slice(&offset.to_le_bytes());
        header[70..72].copy_from_slice(&(self.database.len() as u16).to_le_bytes());
        var_data.extend_from_slice(&database_encoded);
        offset += database_encoded.len() as u16;

        // Client ID (6 bytes at offset 72)
        header[72..78].copy_from_slice(&self.client_id);

        // SSPI (offset 78, length 80) - unused
        header[78..80].copy_from_slice(&offset.to_le_bytes());
        header[80..82].copy_from_slice(&0u16.to_le_bytes());

        // AtchDBFile (offset 82, length 84) - unused
        header[82..84].copy_from_slice(&offset.to_le_bytes());
        header[84..86].copy_from_slice(&0u16.to_le_bytes());

        // ChangePassword (offset 86, length 88) - unused
        header[86..88].copy_from_slice(&offset.to_le_bytes());
        header[88..90].copy_from_slice(&0u16.to_le_bytes());

        // SSPI Long (4 bytes at offset 90)
        header[90..94].copy_from_slice(&0u32.to_le_bytes());

        // Combine header and variable data
        data.extend_from_slice(&header);
        data.extend_from_slice(&var_data);

        // Update total length in header
        let total_len = data.len() as u32;
        data[0..4].copy_from_slice(&total_len.to_le_bytes());

        Ok(data)
    }

    /// Encode password with SQL Server obfuscation
    /// Each byte is XORed with 0x5A and then nibbles are swapped
    fn encode_password(&self, password: &str) -> Vec<u8> {
        use crate::utils::encoding::encode_ucs2_le;

        let password_bytes = encode_ucs2_le(password);
        password_bytes
            .iter()
            .map(|&b| {
                let xored = b ^ 0x5A;
                // Swap high and low nibbles
                ((xored & 0x0F) << 4) | ((xored & 0xF0) >> 4)
            })
            .collect()
    }
}

/// SQL Batch packet - executes a SQL query
#[derive(Debug, Clone)]
pub struct SqlBatchPacket {
    /// SQL query text
    pub sql: String,
    /// Transaction descriptor (0 for non-transactional)
    pub transaction_descriptor: u64,
}

impl SqlBatchPacket {
    /// Create a new SQL Batch packet
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            transaction_descriptor: 0,
        }
    }

    /// Encode the SQL Batch packet to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        use crate::utils::encoding::encode_ucs2_le;

        let mut data = Vec::new();

        // ALL_HEADERS (for transaction descriptor)
        // Total length (4 bytes) - length of all headers
        let headers_length: u32 = 22; // 4 (length) + 18 (transaction header)
        data.extend_from_slice(&headers_length.to_le_bytes());

        // Transaction descriptor header
        // Header type (2 bytes) - 0x0002 for transaction descriptor
        data.extend_from_slice(&2u16.to_le_bytes());
        // Transaction descriptor (8 bytes)
        data.extend_from_slice(&self.transaction_descriptor.to_le_bytes());
        // Outstanding request count (4 bytes) - always 1
        data.extend_from_slice(&1u32.to_le_bytes());

        // SQL text (UCS-2 LE encoded)
        let sql_bytes = encode_ucs2_le(&self.sql);
        data.extend_from_slice(&sql_bytes);

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_header_roundtrip() {
        let header = PacketHeader {
            packet_type: PacketType::PreLogin,
            status: 0x01,
            length: 100,
            spid: 0,
            packet_id: 1,
            window: 0,
        };

        let bytes = header.to_bytes();
        let decoded = PacketHeader::from_bytes(&bytes).unwrap();

        assert_eq!(decoded.packet_type, PacketType::PreLogin);
        assert_eq!(decoded.length, 100);
    }

    #[test]
    fn test_prelogin_packet_encode() {
        let packet = PreLoginPacket::new()
            .with_encryption(EncryptionLevel::EncryptNotSup);

        let bytes = packet.to_bytes().unwrap();

        // Check that it has the required options
        assert!(!bytes.is_empty());

        // First byte should be VERSION token
        assert_eq!(bytes[0], PreLoginOptionToken::Version as u8);
    }

    #[test]
    fn test_prelogin_version_encoding() {
        let packet = PreLoginPacket {
            version: (16, 0, 0x0F, 0x01), // SQL Server 2022 16.0.3840.1
            ..Default::default()
        };

        let version_bytes = packet.encode_version();
        assert_eq!(version_bytes.len(), 6);
        assert_eq!(version_bytes[0], 16); // major
        assert_eq!(version_bytes[1], 0);  // minor
    }

    #[test]
    fn test_encryption_level() {
        let packet = PreLoginPacket::new()
            .with_encryption(EncryptionLevel::EncryptReq);

        assert_eq!(packet.encryption, EncryptionLevel::EncryptReq);
    }

    #[test]
    fn test_login7_packet_creation() {
        let packet = Login7Packet::new(
            "sa".to_string(),
            "Password123".to_string(),
            "master".to_string(),
        );

        assert_eq!(packet.username, "sa");
        assert_eq!(packet.database, "master");
        assert_eq!(packet.app_name, "mssql-driver");
        assert_eq!(packet.client_id.len(), 6);
    }

    #[test]
    fn test_login7_packet_encode() {
        let packet = Login7Packet::new(
            "testuser".to_string(),
            "testpass".to_string(),
            "testdb".to_string(),
        );

        let bytes = packet.to_bytes().unwrap();

        // Check minimum length (94 bytes header + variable data)
        assert!(bytes.len() >= 94);

        // Check TDS version
        let tds_version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert_eq!(tds_version, 0x74000004); // TDS 7.4
    }

    #[test]
    fn test_password_obfuscation() {
        let packet = Login7Packet::new(
            "user".to_string(),
            "pass".to_string(),
            "db".to_string(),
        );

        let encoded = packet.encode_password("test");

        // Password should be obfuscated (not plain text)
        let plain = crate::utils::encoding::encode_ucs2_le("test");
        assert_ne!(encoded, plain);

        // Should have same length
        assert_eq!(encoded.len(), plain.len());
    }
}
