//! Controller implementation

use crate::error::{Error, Result};
use scm_core::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Device identifier
    pub id: String,
    /// Connection timestamp
    pub connected_at: std::time::SystemTime,
    /// Last message timestamp
    pub last_seen: std::time::SystemTime,
}

/// SCM Controller
///
/// Manages connections with multiple devices and routes messages.
pub struct Controller {
    /// Controller identifier
    controller_id: String,
    /// Connected devices
    devices: Arc<Mutex<HashMap<String, DeviceInfo>>>,
    /// Next message ID
    next_id: Arc<Mutex<u16>>,
}

impl Controller {
    /// Create a new controller
    pub fn new(controller_id: impl Into<String>) -> Self {
        Self {
            controller_id: controller_id.into(),
            devices: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Get controller ID
    pub fn controller_id(&self) -> &str {
        &self.controller_id
    }

    /// Get number of connected devices
    pub async fn device_count(&self) -> usize {
        self.devices.lock().await.len()
    }

    /// Get list of connected devices
    pub async fn devices(&self) -> Vec<DeviceInfo> {
        self.devices.lock().await.values().cloned().collect()
    }

    /// Register a new device
    pub async fn register_device(&self, device_id: String) -> Result<()> {
        let now = std::time::SystemTime::now();
        let info = DeviceInfo {
            id: device_id.clone(),
            connected_at: now,
            last_seen: now,
        };

        self.devices.lock().await.insert(device_id, info);
        Ok(())
    }

    /// Unregister a device
    pub async fn unregister_device(&self, device_id: &str) -> Result<()> {
        self.devices
            .lock()
            .await
            .remove(device_id)
            .ok_or_else(|| Error::DeviceNotFound(device_id.to_string()))?;
        Ok(())
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

    /// Send a COMMAND message
    pub async fn send_command(&self, payload: scm_core::Payload) -> Result<Message> {
        let id = self.next_message_id().await;
        Ok(Message::command(id, payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_controller_creation() {
        let controller = Controller::new("test-controller");
        assert_eq!(controller.controller_id(), "test-controller");
        assert_eq!(controller.device_count().await, 0);
    }

    #[tokio::test]
    async fn test_device_registration() {
        let controller = Controller::new("test");
        controller.register_device("device-001".to_string()).await.unwrap();
        assert_eq!(controller.device_count().await, 1);

        let devices = controller.devices().await;
        assert_eq!(devices[0].id, "device-001");
    }

    #[tokio::test]
    async fn test_device_unregistration() {
        let controller = Controller::new("test");
        controller.register_device("device-001".to_string()).await.unwrap();
        controller.unregister_device("device-001").await.unwrap();
        assert_eq!(controller.device_count().await, 0);
    }
}
