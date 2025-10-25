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
        let mut offset: u16 = 0;
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

        offset = (num_options * 5 + 1) as u16; // +1 for terminator

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
}
