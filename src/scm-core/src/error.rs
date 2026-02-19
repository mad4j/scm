//! Error types for SCM protocol

use thiserror::Error;

/// Result type for SCM operations
pub type Result<T> = std::result::Result<T, Error>;

/// SCM Protocol Errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    /// Invalid message format
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    /// CRC checksum mismatch
    #[error("CRC checksum mismatch: expected {expected:#06x}, got {actual:#06x}")]
    CrcMismatch { expected: u16, actual: u16 },

    /// Unsupported message type
    #[error("Unsupported message type: {0:#04x}")]
    UnsupportedMessageType(u8),

    /// Message too short
    #[error("Message too short: expected at least {expected} bytes, got {actual}")]
    MessageTooShort { expected: usize, actual: usize },

    /// Invalid frame length
    #[error("Invalid frame length: {0}")]
    InvalidLength(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),
}

/// SCM Protocol Error Codes (as defined in specification)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// Invalid message format
    InvalidFormat = 0x01,
    /// CRC checksum mismatch
    CrcMismatch = 0x02,
    /// Unsupported message type
    UnsupportedType = 0x03,
    /// Timeout
    Timeout = 0x04,
    /// Device not ready
    DeviceNotReady = 0x05,
    /// Command execution failed
    CommandFailed = 0x10,
}

impl ErrorCode {
    /// Convert error code to u8
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Try to convert u8 to error code
    pub fn from_u8(code: u8) -> Option<Self> {
        match code {
            0x01 => Some(ErrorCode::InvalidFormat),
            0x02 => Some(ErrorCode::CrcMismatch),
            0x03 => Some(ErrorCode::UnsupportedType),
            0x04 => Some(ErrorCode::Timeout),
            0x05 => Some(ErrorCode::DeviceNotReady),
            0x10 => Some(ErrorCode::CommandFailed),
            _ => None,
        }
    }
}
