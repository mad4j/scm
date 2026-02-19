//! # SCM Device
//!
//! Device-side implementation for Simple Communication Middleware.
//!
//! This crate provides the device client functionality for connecting to
//! a controller and exchanging messages according to the SCM protocol.
//!
//! ## Example
//!
//! ```rust,no_run
//! use scm_device::DeviceClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create device client
//!     let client = DeviceClient::new("device-001");
//!
//!     // Connect to controller
//!     // client.connect("127.0.0.1:8080").await?;
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;

pub use client::DeviceClient;
pub use error::{Error, Result};
