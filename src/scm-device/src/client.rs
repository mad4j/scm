//! Device client implementation

use crate::error::Result;
use scm_core::Message;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Device connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected
    Disconnected,
    /// Connecting in progress
    Connecting,
    /// Connected and ready
    Connected,
}

/// SCM Device Client
///
/// Represents a device that communicates with a controller using the SCM protocol.
pub struct DeviceClient {
    /// Device identifier
    device_id: String,
    /// Connection state
    state: Arc<Mutex<ConnectionState>>,
    /// Next message ID
    next_id: Arc<Mutex<u16>>,
}

impl DeviceClient {
    /// Create a new device client
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Get device ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Get connection state
    pub async fn state(&self) -> ConnectionState {
        *self.state.lock().await
    }

    /// Get next message ID
    async fn next_message_id(&self) -> u16 {
        let mut id = self.next_id.lock().await;
        let current = *id;
        *id = id.wrapping_add(1);
        current
    }

    /// Send a PING message
    pub async fn ping(&self) -> Result<Message> {
        let id = self.next_message_id().await;
        Ok(Message::ping(id))
    }

    /// Send a DATA message
    pub async fn send_data(&self, payload: scm_core::Payload) -> Result<Message> {
        let id = self.next_message_id().await;
        Ok(Message::data(id, payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_device_creation() {
        let client = DeviceClient::new("test-device");
        assert_eq!(client.device_id(), "test-device");
        assert_eq!(client.state().await, ConnectionState::Disconnected);
    }

    #[tokio::test]
    async fn test_message_id_increment() {
        let client = DeviceClient::new("test");
        let id1 = client.next_message_id().await;
        let id2 = client.next_message_id().await;
        assert_eq!(id2, id1.wrapping_add(1));
    }
}
