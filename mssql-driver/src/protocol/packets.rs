//! TDS packet structures and definitions

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
}
