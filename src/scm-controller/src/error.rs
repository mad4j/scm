//! Controller error types

use thiserror::Error;

/// Result type for controller operations
pub type Result<T> = std::result::Result<T, Error>;

/// Controller errors
#[derive(Error, Debug)]
pub enum Error {
    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(#[from] scm_core::Error),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Device not found
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// Maximum connections reached
    #[error("Maximum connections reached")]
    MaxConnectionsReached,
}
