//! Protocol types and constants

/// Start of Text byte (STX) - Frame delimiter
pub const STX: u8 = 0x02;

/// Message type identifiers
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Keep-alive message (bidirectional)
    Ping = 0x01,
    /// Response to PING (bidirectional)
    Pong = 0x02,
    /// Data transfer (bidirectional)
    Data = 0x10,
    /// Command message (Controller → Device)
    Command = 0x11,
    /// Acknowledgment (bidirectional)
    Ack = 0x20,
    /// Negative acknowledgment (bidirectional)
    Nack = 0x21,
    /// Error message (bidirectional)
    Error = 0xFF,
}

impl MessageType {
    /// Convert message type to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Try to convert u8 to message type
    pub fn from_u8(type_code: u8) -> Option<Self> {
        match type_code {
            0x01 => Some(MessageType::Ping),
            0x02 => Some(MessageType::Pong),
            0x10 => Some(MessageType::Data),
            0x11 => Some(MessageType::Command),
            0x20 => Some(MessageType::Ack),
            0x21 => Some(MessageType::Nack),
            0xFF => Some(MessageType::Error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_conversion() {
        assert_eq!(MessageType::Ping.as_u8(), 0x01);
        assert_eq!(MessageType::from_u8(0x01), Some(MessageType::Ping));
        assert_eq!(MessageType::from_u8(0x99), None);
    }
}
