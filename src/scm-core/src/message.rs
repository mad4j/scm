//! Message structure and handling

use crate::error::{Error, Result};
use crate::types::MessageType;
use serde::{Deserialize, Serialize};

/// Message payload data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload {
    /// Empty payload
    Empty,
    /// Raw bytes
    Raw(Vec<u8>),
    /// JSON data
    Json(serde_json::Value),
}

impl Payload {
    /// Create empty payload
    pub fn empty() -> Self {
        Payload::Empty
    }

    /// Create payload from bytes
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Payload::Raw(data)
    }

    /// Create payload from JSON
    pub fn from_json(value: serde_json::Value) -> Self {
        Payload::Json(value)
    }

    /// Convert payload to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        match self {
            Payload::Empty => Ok(Vec::new()),
            Payload::Raw(data) => Ok(data.clone()),
            Payload::Json(value) => {
                serde_json::to_vec(value)
                    .map_err(|e| Error::SerializationError(e.to_string()))
            }
        }
    }

    /// Get payload length in bytes
    pub fn len(&self) -> usize {
        match self {
            Payload::Empty => 0,
            Payload::Raw(data) => data.len(),
            Payload::Json(value) => {
                serde_json::to_vec(value).map(|v| v.len()).unwrap_or(0)
            }
        }
    }

    /// Check if payload is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// SCM Protocol Message
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Message type
    pub msg_type: MessageType,
    /// Message ID for acknowledgment
    pub id: u16,
    /// Message payload
    pub payload: Payload,
}

impl Message {
    /// Create a new message
    pub fn new(msg_type: MessageType, id: u16, payload: Payload) -> Self {
        Self {
            msg_type,
            id,
            payload,
        }
    }

    /// Create a PING message
    pub fn ping(id: u16) -> Self {
        Self::new(MessageType::Ping, id, Payload::empty())
    }

    /// Create a PONG message
    pub fn pong(id: u16) -> Self {
        Self::new(MessageType::Pong, id, Payload::empty())
    }

    /// Create an ACK message
    pub fn ack(id: u16) -> Self {
        Self::new(MessageType::Ack, id, Payload::empty())
    }

    /// Create a NACK message
    pub fn nack(id: u16) -> Self {
        Self::new(MessageType::Nack, id, Payload::empty())
    }

    /// Create a DATA message
    pub fn data(id: u16, payload: Payload) -> Self {
        Self::new(MessageType::Data, id, payload)
    }

    /// Create a COMMAND message
    pub fn command(id: u16, payload: Payload) -> Self {
        Self::new(MessageType::Command, id, payload)
    }

    /// Create an ERROR message
    pub fn error(id: u16, payload: Payload) -> Self {
        Self::new(MessageType::Error, id, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_payload() {
        let payload = Payload::empty();
        assert!(payload.is_empty());
        assert_eq!(payload.len(), 0);
    }

    #[test]
    fn test_raw_payload() {
        let data = vec![1, 2, 3, 4];
        let payload = Payload::from_bytes(data.clone());
        assert_eq!(payload.len(), 4);
        assert_eq!(payload.to_bytes().unwrap(), data);
    }

    #[test]
    fn test_ping_message() {
        let msg = Message::ping(1);
        assert_eq!(msg.msg_type, MessageType::Ping);
        assert_eq!(msg.id, 1);
        assert!(msg.payload.is_empty());
    }

    #[test]
    fn test_data_message() {
        let payload = Payload::from_bytes(vec![1, 2, 3]);
        let msg = Message::data(42, payload.clone());
        assert_eq!(msg.msg_type, MessageType::Data);
        assert_eq!(msg.id, 42);
        assert_eq!(msg.payload, payload);
    }
}
