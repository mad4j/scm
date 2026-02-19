//! # SCM Core
//!
//! Core protocol implementation for Simple Communication Middleware.
//!
//! This crate provides the fundamental building blocks for the SCM protocol:
//! - Message structure and serialization
//! - Frame encoding/decoding with CRC validation
//! - Protocol types and constants
//! - Error handling
//!
//! ## Example
//!
//! ```rust
//! use scm_core::{Message, MessageType, Frame, Payload};
//!
//! // Create a PING message
//! let message = Message::new(MessageType::Ping, 1, Payload::empty());
//!
//! // Encode to frame
//! let frame = Frame::from_message(&message).unwrap();
//!
//! // Serialize to bytes
//! let bytes = frame.to_bytes();
//! ```

pub mod error;
pub mod frame;
pub mod message;
pub mod types;

pub use error::{Error, Result};
pub use frame::Frame;
pub use message::{Message, Payload};
pub use types::{MessageType, STX};
