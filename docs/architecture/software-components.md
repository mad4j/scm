# SCM Software Architecture

## Simple Communication Middleware - Software Component Architecture

### 1. Architecture Overview

The SCM software architecture is designed as a modular, layered system that separates concerns and enables independent development and testing of components.

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│              (Device App / HMI Controller)                   │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────┴────────────────────────────────────┐
│                     SCM API Layer                            │
│            (High-level Protocol Interface)                   │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────┴────────────────────────────────────┐
│                  Protocol Core Layer                         │
│         (Message Format, Parsing, Validation)                │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────┴────────────────────────────────────┐
│                   Transport Layer                            │
│            (UART, TCP, UDP, CAN, USB)                        │
└─────────────────────────────────────────────────────────────┘
```

### 2. Component Breakdown

#### 2.1 SCM Core (`scm-core`)

**Responsibility**: Core protocol implementation, message handling, and state management.

**Modules**:
- `message`: Message structure and serialization/deserialization
- `frame`: Frame encoding/decoding with CRC validation
- `protocol`: Protocol state machine and message routing
- `error`: Error types and handling
- `types`: Common data types and constants

**Key Features**:
- Message framing and deframing
- CRC-16/CCITT checksum calculation
- Protocol state machine
- Message validation
- Error handling and reporting

**Dependencies**:
- No external runtime dependencies (core protocol only)
- Optional: `serde` for JSON payload handling
- Optional: `log` for logging

#### 2.2 SCM Device (`scm-device`)

**Responsibility**: Device-side implementation for embedded systems and IoT devices.

**Modules**:
- `client`: Device client implementation
- `handler`: Message handler and callback interface
- `commands`: Command execution framework
- `sensors`: Sensor data abstraction
- `connection`: Connection management and reconnection logic

**Key Features**:
- Automatic connection establishment
- Command handler registration
- Sensor data publishing
- Heartbeat/keep-alive mechanism
- Automatic reconnection on disconnect

**Dependencies**:
- `scm-core`: Core protocol implementation
- `tokio` or `async-std`: Async runtime (optional)
- Platform-specific serial/network libraries

#### 2.3 SCM Controller (`scm-controller`)

**Responsibility**: Control node/HMI implementation for monitoring and controlling devices.

**Modules**:
- `server`: Controller server implementation
- `device_manager`: Multi-device management
- `command_api`: Command sending interface
- `data_collector`: Data collection and storage
- `events`: Event notification system

**Key Features**:
- Multi-device connection management
- Command dispatch to devices
- Real-time data collection
- Event-driven architecture
- Device discovery and registration

**Dependencies**:
- `scm-core`: Core protocol implementation
- `tokio` or `async-std`: Async runtime
- Network/transport libraries

### 3. Data Flow

#### 3.1 Outbound Message Flow (Device → Controller)

```
Sensor/Application
      ↓
  Data Collection
      ↓
  Message Creation (scm-device)
      ↓
  Protocol Encoding (scm-core)
      ↓
  Frame Serialization (scm-core)
      ↓
  CRC Calculation (scm-core)
      ↓
  Transport Send (scm-device)
      ↓
   Network Layer
```

#### 3.2 Inbound Message Flow (Controller → Device)

```
   Network Layer
      ↓
  Transport Receive (scm-device)
      ↓
  Frame Deserialization (scm-core)
      ↓
  CRC Validation (scm-core)
      ↓
  Protocol Decoding (scm-core)
      ↓
  Message Routing (scm-device)
      ↓
  Command Handler Execution
      ↓
  Application/Actuator
```

### 4. State Management

#### 4.1 Connection States

```
┌──────────────┐
│ Disconnected │
└───────┬──────┘
        │
        ↓ (connect)
┌───────────────┐
│  Connecting   │
└───────┬───────┘
        │
        ↓ (ping/pong)
┌───────────────┐
│   Connected   │
└───────┬───────┘
        │
        ↓ (timeout/error)
┌──────────────┐
│ Disconnected │
└──────────────┘
```

#### 4.2 Message States

```
┌─────────┐
│  Created│
└────┬────┘
     │
     ↓ (send)
┌─────────┐
│ Pending │
└────┬────┘
     │
     ├─(ack)──► ┌───────────┐
     │          │ Confirmed │
     │          └───────────┘
     │
     └─(nack)─► ┌──────────┐
                │  Rejected│
                └──────────┘
```

### 5. Threading Model

#### 5.1 Device Side

```
Main Thread:
  - Application logic
  - Sensor reading
  - Command handling

I/O Thread:
  - Message sending
  - Message receiving
  - Keep-alive

Background Thread:
  - Connection monitoring
  - Retry logic
```

#### 5.2 Controller Side

```
Main Thread:
  - Application/UI logic
  - Command dispatch

I/O Thread Pool:
  - Per-device connections
  - Message routing
  - Event dispatch

Worker Threads:
  - Data processing
  - Storage operations
```

### 6. Error Handling Strategy

#### 6.1 Error Categories

1. **Transport Errors**: Connection failures, timeouts
2. **Protocol Errors**: Invalid messages, CRC failures
3. **Application Errors**: Command execution failures

#### 6.2 Error Recovery

- **Transient Errors**: Automatic retry with backoff
- **Protocol Errors**: Report and continue
- **Fatal Errors**: Close connection and notify application

### 7. Configuration

#### 7.1 Device Configuration

```toml
[device]
id = "device-001"
name = "Temperature Sensor"

[connection]
transport = "serial"
port = "/dev/ttyUSB0"
baud_rate = 115200

[protocol]
timeout = 5000  # milliseconds
max_retries = 3
keepalive_interval = 30000  # milliseconds
```

#### 7.2 Controller Configuration

```toml
[controller]
name = "Main Controller"
port = 8080

[devices]
max_connections = 100
discovery_enabled = true

[protocol]
timeout = 5000
max_retries = 3
```

### 8. Testing Strategy

#### 8.1 Unit Tests

- Message serialization/deserialization
- CRC calculation
- Protocol state transitions
- Error handling

#### 8.2 Integration Tests

- End-to-end message flow
- Multi-device scenarios
- Error recovery
- Performance testing

#### 8.3 Mock Transport

- In-memory transport for testing
- Simulated network conditions
- Fault injection

### 9. Performance Considerations

#### 9.1 Throughput

- Target: 1000 messages/second per device
- Batch message processing
- Zero-copy serialization where possible

#### 9.2 Latency

- Target: < 100ms end-to-end latency
- Minimal message processing overhead
- Efficient I/O handling

#### 9.3 Memory

- Fixed-size message buffers
- Connection pooling
- Efficient data structures

### 10. Deployment Architectures

#### 10.1 Embedded Device

```
┌────────────────────────────┐
│   Microcontroller          │
│  ┌──────────────────────┐  │
│  │  Application         │  │
│  └──────┬───────────────┘  │
│         │                  │
│  ┌──────┴───────────────┐  │
│  │  SCM Device          │  │
│  └──────┬───────────────┘  │
│         │                  │
│  ┌──────┴───────────────┐  │
│  │  UART Driver         │  │
│  └──────────────────────┘  │
└────────────────────────────┘
```

#### 10.2 Gateway/Controller

```
┌─────────────────────────────────┐
│   Linux Server                  │
│  ┌───────────────────────────┐  │
│  │  HMI Application          │  │
│  └──────┬────────────────────┘  │
│         │                       │
│  ┌──────┴────────────────────┐  │
│  │  SCM Controller           │  │
│  │  - Device Manager         │  │
│  │  - Data Collector         │  │
│  └──────┬────────────────────┘  │
│         │                       │
│  ┌──────┴────────────────────┐  │
│  │  TCP/Serial Transport     │  │
│  └───────────────────────────┘  │
└─────────────────────────────────┘
```

### 11. Extension Points

- Custom message types
- Custom transport implementations
- Custom error handlers
- Custom data serialization formats
- Plugin architecture for commands
