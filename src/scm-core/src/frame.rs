//! Frame encoding and decoding with CRC validation

use crate::error::{Error, Result};
use crate::message::Message;
use crate::types::{MessageType, STX};

/// Minimum frame size (STX + LEN + TYPE + ID + CRC)
const MIN_FRAME_SIZE: usize = 8;

/// SCM Protocol Frame
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// Start of text marker
    pub stx: u8,
    /// Frame length (including STX and CRC)
    pub len: u16,
    /// Message type
    pub msg_type: u8,
    /// Message ID
    pub id: u16,
    /// Payload data
    pub payload: Vec<u8>,
    /// CRC checksum
    pub crc: u16,
}

impl Frame {
    /// Create a frame from a message
    pub fn from_message(message: &Message) -> Result<Self> {
        let payload = message.payload.to_bytes()?;
        let len = (MIN_FRAME_SIZE + payload.len()) as u16;

        let mut frame = Self {
            stx: STX,
            len,
            msg_type: message.msg_type.as_u8(),
            id: message.id,
            payload,
            crc: 0,
        };

        // Calculate CRC over all fields except CRC itself
        frame.crc = frame.calculate_crc();

        Ok(frame)
    }

    /// Convert frame to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len as usize);

        bytes.push(self.stx);
        bytes.extend_from_slice(&self.len.to_be_bytes());
        bytes.push(self.msg_type);
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.crc.to_be_bytes());

        bytes
    }

    /// Parse a frame from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < MIN_FRAME_SIZE {
            return Err(Error::MessageTooShort {
                expected: MIN_FRAME_SIZE,
                actual: bytes.len(),
            });
        }

        // Check STX
        if bytes[0] != STX {
            return Err(Error::InvalidFormat(format!(
                "Invalid STX: expected {:#04x}, got {:#04x}",
                STX, bytes[0]
            )));
        }

        // Parse length
        let len = u16::from_be_bytes([bytes[1], bytes[2]]);

        if bytes.len() != len as usize {
            return Err(Error::InvalidLength(format!(
                "Frame length mismatch: header says {}, actual {}",
                len,
                bytes.len()
            )));
        }

        // Parse type
        let msg_type = bytes[3];

        // Parse ID
        let id = u16::from_be_bytes([bytes[4], bytes[5]]);

        // Parse payload
        let payload_len = len as usize - MIN_FRAME_SIZE;
        let payload = bytes[6..6 + payload_len].to_vec();

        // Parse CRC
        let crc_offset = 6 + payload_len;
        let crc = u16::from_be_bytes([bytes[crc_offset], bytes[crc_offset + 1]]);

        let frame = Self {
            stx: STX,
            len,
            msg_type,
            id,
            payload,
            crc,
        };

        // Validate CRC
        let calculated_crc = frame.calculate_crc();
        if calculated_crc != crc {
            return Err(Error::CrcMismatch {
                expected: crc,
                actual: calculated_crc,
            });
        }

        Ok(frame)
    }

    /// Convert frame to message
    pub fn to_message(&self) -> Result<Message> {
        let msg_type = MessageType::from_u8(self.msg_type)
            .ok_or_else(|| Error::UnsupportedMessageType(self.msg_type))?;

        let payload = if self.payload.is_empty() {
            crate::message::Payload::empty()
        } else {
            crate::message::Payload::from_bytes(self.payload.clone())
        };

        Ok(Message::new(msg_type, self.id, payload))
    }

    /// Calculate CRC-16/CCITT checksum
    ///
    /// This implementation uses the CCITT polynomial: 0x1021
    /// Initial value: 0xFFFF
    fn calculate_crc(&self) -> u16 {
        let mut crc: u16 = 0xFFFF;

        // CRC over: STX, LEN, TYPE, ID, PAYLOAD
        crc = Self::update_crc(crc, self.stx);
        crc = Self::update_crc(crc, (self.len >> 8) as u8);
        crc = Self::update_crc(crc, (self.len & 0xFF) as u8);
        crc = Self::update_crc(crc, self.msg_type);
        crc = Self::update_crc(crc, (self.id >> 8) as u8);
        crc = Self::update_crc(crc, (self.id & 0xFF) as u8);

        for byte in &self.payload {
            crc = Self::update_crc(crc, *byte);
        }

        crc
    }

    /// Update CRC with one byte using CCITT polynomial
    #[inline]
    fn update_crc(crc: u16, byte: u8) -> u16 {
        let mut crc = crc ^ ((byte as u16) << 8);
        for _ in 0..8 {
            if (crc & 0x8000) != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
        crc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Payload;

    #[test]
    fn test_frame_creation() {
        let msg = Message::ping(1);
        let frame = Frame::from_message(&msg).unwrap();

        assert_eq!(frame.stx, STX);
        assert_eq!(frame.msg_type, MessageType::Ping.as_u8());
        assert_eq!(frame.id, 1);
        assert!(frame.payload.is_empty());
    }

    #[test]
    fn test_frame_serialization() {
        let msg = Message::ping(1);
        let frame = Frame::from_message(&msg).unwrap();
        let bytes = frame.to_bytes();

        assert_eq!(bytes[0], STX);
        assert_eq!(bytes.len(), MIN_FRAME_SIZE);
    }

    #[test]
    fn test_frame_deserialization() {
        let msg = Message::ping(42);
        let frame = Frame::from_message(&msg).unwrap();
        let bytes = frame.to_bytes();

        let parsed_frame = Frame::from_bytes(&bytes).unwrap();
        assert_eq!(parsed_frame, frame);
    }

    #[test]
    fn test_frame_roundtrip() {
        let payload = Payload::from_bytes(vec![1, 2, 3, 4, 5]);
        let msg = Message::data(100, payload);
        let frame = Frame::from_message(&msg).unwrap();
        let bytes = frame.to_bytes();

        let parsed_frame = Frame::from_bytes(&bytes).unwrap();
        let parsed_msg = parsed_frame.to_message().unwrap();

        assert_eq!(parsed_msg.msg_type, msg.msg_type);
        assert_eq!(parsed_msg.id, msg.id);
        assert_eq!(parsed_msg.payload, msg.payload);
    }

    #[test]
    fn test_invalid_stx() {
        let bytes = vec![0xFF, 0, 8, 0x01, 0, 1, 0, 0];
        let result = Frame::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_crc_mismatch() {
        let msg = Message::ping(1);
        let frame = Frame::from_message(&msg).unwrap();
        let mut bytes = frame.to_bytes();

        // Corrupt the CRC
        let len = bytes.len();
        bytes[len - 1] ^= 0xFF;

        let result = Frame::from_bytes(&bytes);
        assert!(matches!(result, Err(Error::CrcMismatch { .. })));
    }
}
