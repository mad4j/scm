//! Device client error types

use thiserror::Error;

/// Result type for device operations
pub type Result<T> = std::result::Result<T, Error>;

/// Device errors
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

    /// Timeout
    #[error("Operation timed out")]
    Timeout,

    /// Not connected
    #[error("Not connected")]
    NotConnected,
}
