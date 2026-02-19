//! # SCM Controller
//!
//! Controller/HMI implementation for Simple Communication Middleware.
//!
//! This crate provides the controller server functionality for managing
//! connections with multiple devices and exchanging messages according to
//! the SCM protocol.
//!
//! ## Example
//!
//! ```rust,no_run
//! use scm_controller::Controller;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create controller
//!     let controller = Controller::new("main-controller");
//!
//!     // Start listening for device connections
//!     // controller.listen("0.0.0.0:8080").await?;
//!
//!     Ok(())
//! }
//! ```

pub mod controller;
pub mod error;

pub use controller::Controller;
pub use error::{Error, Result};
