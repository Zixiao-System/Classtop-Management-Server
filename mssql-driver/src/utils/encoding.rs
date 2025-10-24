//! Character encoding utilities for SQL Server (UCS-2 LE)

use crate::error::{Error, Result};

/// Encode string to UCS-2 LE (SQL Server internal encoding)
pub fn encode_ucs2_le(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()
}

/// Decode UCS-2 LE bytes to String
pub fn decode_ucs2_le(bytes: &[u8]) -> Result<String> {
    if bytes.len() % 2 != 0 {
        return Err(Error::EncodingError(
            "Invalid UCS-2 LE byte sequence (odd length)".to_string(),
        ));
    }

    let utf16_chars: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16(&utf16_chars)
        .map_err(|e| Error::EncodingError(format!("Invalid UTF-16 sequence: {}", e)))
}

/// Encode string with length prefix (B_VARCHAR)
pub fn encode_b_varchar(s: &str) -> Vec<u8> {
    let encoded = encode_ucs2_le(s);
    let len = (encoded.len() / 2) as u8; // Character count, not byte count

    let mut result = Vec::with_capacity(1 + encoded.len());
    result.push(len);
    result.extend_from_slice(&encoded);
    result
}

/// Encode string with 16-bit length prefix (US_VARCHAR)
pub fn encode_us_varchar(s: &str) -> Vec<u8> {
    let encoded = encode_ucs2_le(s);
    let len = (encoded.len() / 2) as u16; // Character count

    let mut result = Vec::with_capacity(2 + encoded.len());
    result.extend_from_slice(&len.to_le_bytes());
    result.extend_from_slice(&encoded);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ucs2_le_encoding() {
        let input = "hello";
        let encoded = encode_ucs2_le(input);

        // "hello" in UCS-2 LE
        assert_eq!(
            encoded,
            vec![
                0x68, 0x00, // h
                0x65, 0x00, // e
                0x6C, 0x00, // l
                0x6C, 0x00, // l
                0x6F, 0x00, // o
            ]
        );

        let decoded = decode_ucs2_le(&encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_chinese_encoding() {
        let input = "你好";
        let encoded = encode_ucs2_le(input);
        let decoded = decode_ucs2_le(&encoded).unwrap();
        assert_eq!(decoded, input);
    }

    #[test]
    fn test_b_varchar() {
        let input = "test";
        let encoded = encode_b_varchar(input);

        // First byte should be length (4 characters)
        assert_eq!(encoded[0], 4);
        assert_eq!(encoded.len(), 1 + 4 * 2); // 1 byte length + 4 chars * 2 bytes
    }
}
