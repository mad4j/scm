# SCM - Simple Communication Middleware

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

A lightweight, bidirectional communication protocol designed for communication between embedded devices and control nodes implementing Human-Machine Interface (HMI) functionality.

## Overview

SCM (Simple Communication Middleware) is a protocol and software architecture designed to provide:

- **Simplicity**: Easy to implement on resource-constrained devices
- **Reliability**: Built-in acknowledgment and error handling with CRC-16 validation
- **Flexibility**: Extensible message format for various device types
- **Efficiency**: Minimal overhead for real-time communication

## Project Structure

```
scm/
├── docs/                           # Documentation
│   ├── protocol/                   # Protocol specification
│   │   └── specification.md        # Protocol v1.0 specification
│   ├── architecture/               # Software architecture
│   │   └── software-components.md  # Component architecture
│   └── issues/                     # Project planning
│       └── issue-hierarchy.md      # Issue tracking structure
├── src/                            # Source code
│   ├── scm-core/                   # Core protocol implementation
│   ├── scm-device/                 # Device-side implementation
│   └── scm-controller/             # Controller/HMI implementation
├── examples/                       # Example applications
└── tests/                          # Integration tests
```

## Components

### 1. Protocol Specification (`docs/protocol/`)

RFC-style specification document (SCM-SPEC-001) covering:
- Operational environment: Frontend (HMI/View) and Backend (apparatus) components
- Four commands: `configure`, `query`, `execute` (Frontend → Backend) and `notify` (Backend → Frontend)
- Message encoding via Protocol Buffers over WebSocket
- Dispatcher component and its API (Rust on Frontend, Rust or C/C++ on Backend)

📖 [Read the Protocol Specification](docs/protocol/specification.md)
📄 [Protocol Buffer Definitions](docs/protocol/scm.proto)

### 2. Software Architecture (`docs/architecture/`)

Detailed software component architecture including:
- Component layering and module structure
- Data flow and message routing
- State management
- Threading model
- Configuration and deployment

📖 [Read the Architecture Document](docs/architecture/software-components.md)

### 3. Reference Implementation (`src/`)

Rust implementation consisting of three main crates:

#### `scm-core`
Core protocol implementation providing:
- Message structure and serialization
- Frame encoding/decoding with CRC-16/CCITT
- Protocol types and constants
- Error handling

#### `scm-device`
Device-side implementation for embedded systems:
- Device client implementation
- Connection management
- Command handler framework
- Automatic reconnection

#### `scm-controller`
Controller/HMI implementation:
- Multi-device connection management
- Command dispatch system
- Data collection framework
- Event notification system

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building

```bash
# Clone the repository
git clone https://github.com/mad4j/scm.git
cd scm

# Build all components
cargo build --release

# Run tests
cargo test

# Build documentation
cargo doc --no-deps --open
```

### Example Usage

#### Device Side

```rust
use scm_device::DeviceClient;
use scm_core::Payload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create device client
    let client = DeviceClient::new("temperature-sensor-001");
    
    // Send data
    let data = Payload::from_json(serde_json::json!({
        "timestamp": 1234567890,
        "sensor_id": "temp-01",
        "value": 23.5,
        "unit": "celsius"
    }));
    
    let message = client.send_data(data).await?;
    
    Ok(())
}
```

#### Controller Side

```rust
use scm_controller::Controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create controller
    let controller = Controller::new("main-controller");
    
    // Register device
    controller.register_device("temperature-sensor-001".to_string()).await?;
    
    // Get device count
    println!("Connected devices: {}", controller.device_count().await);
    
    Ok(())
}
```

## Development Roadmap

The project is organized into 8 main epics:

1. ✅ **Protocol Specification** - Define complete protocol specification
2. ✅ **Software Architecture** - Design component architecture
3. 🔄 **Core Implementation** - Implement core protocol (in progress)
4. 📋 **Device Implementation** - Implement device functionality
5. 📋 **Controller Implementation** - Implement controller functionality
6. 📋 **Examples and Tools** - Create examples and utilities
7. 📋 **Testing and QA** - Comprehensive testing
8. 📋 **Documentation** - Complete documentation

📖 [View Complete Issue Hierarchy](docs/issues/issue-hierarchy.md)

## Documentation

- [Protocol Specification](docs/protocol/specification.md) - SCM-SPEC-001 (RFC-style, Draft)
- [Protocol Buffer Definitions](docs/protocol/scm.proto) - Proto3 message stubs
- [Software Architecture](docs/architecture/software-components.md) - Component design
- [Issue Hierarchy](docs/issues/issue-hierarchy.md) - Project planning and tracking
- [API Documentation](https://docs.rs/scm-core) - Generated API docs (coming soon)

## Contributing

Contributions are welcome! Please read the issue hierarchy and documentation before starting work.

### Development Guidelines

1. Follow the existing code style
2. Add tests for new functionality
3. Update documentation for user-facing changes
4. Ensure all tests pass before submitting PR

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Daniele Olmisani

## Project Status

🚧 **In Development** - Version 0.1.0

The project structure is established with:
- ✅ Complete protocol specification
- ✅ Detailed software architecture
- ✅ Reference implementation skeleton
- ✅ Project planning and issue hierarchy
- 🔄 Core implementation in progress
