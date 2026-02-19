# SCM Protocol Specification

## Simple Communication Middleware Protocol v1.0

### 1. Overview

The SCM (Simple Communication Middleware) protocol defines a lightweight, bidirectional communication protocol between devices and control nodes implementing Human-Machine Interface (HMI) functionality.

### 2. Protocol Goals

- **Simplicity**: Easy to implement on resource-constrained devices
- **Reliability**: Built-in acknowledgment and error handling
- **Flexibility**: Extensible message format for various device types
- **Efficiency**: Minimal overhead for real-time communication

### 3. Architecture

```
┌─────────────┐          SCM Protocol          ┌──────────────────┐
│   Device    │ ◄─────────────────────────────► │ Control Node/HMI │
│  (Embedded) │                                 │                  │
└─────────────┘                                 └──────────────────┘
```

### 4. Message Format

#### 4.1 Frame Structure

All messages follow this structure:

```
┌──────┬──────┬─────────┬─────────┬─────────┬──────────┐
│ STX  │ LEN  │  TYPE   │   ID    │ PAYLOAD │   CRC    │
├──────┼──────┼─────────┼─────────┼─────────┼──────────┤
│ 1B   │ 2B   │   1B    │   2B    │  N bytes│   2B     │
└──────┴──────┴─────────┴─────────┴─────────┴──────────┘
```

- **STX** (Start of Text): 0x02 - Frame delimiter
- **LEN**: 16-bit length of entire frame (including STX and CRC)
- **TYPE**: Message type identifier
- **ID**: Message/sequence identifier for acknowledgment
- **PAYLOAD**: Variable-length message data
- **CRC**: 16-bit CRC-16/CCITT checksum

#### 4.2 Message Types

| Type | Code | Description | Direction |
|------|------|-------------|-----------|
| PING | 0x01 | Keep-alive message | Bidirectional |
| PONG | 0x02 | Response to PING | Bidirectional |
| DATA | 0x10 | Data transfer | Bidirectional |
| CMD  | 0x11 | Command message | Controller → Device |
| ACK  | 0x20 | Acknowledgment | Bidirectional |
| NACK | 0x21 | Negative acknowledgment | Bidirectional |
| ERR  | 0xFF | Error message | Bidirectional |

### 5. Communication Flow

#### 5.1 Connection Establishment

```
Device                           Controller
  │                                  │
  │◄─────────── PING ────────────────│
  │                                  │
  │──────────── PONG ───────────────►│
  │                                  │
```

#### 5.2 Data Transfer with Acknowledgment

```
Device                           Controller
  │                                  │
  │──────────── DATA ───────────────►│
  │                                  │
  │◄─────────── ACK ─────────────────│
  │                                  │
```

#### 5.3 Command Execution

```
Device                           Controller
  │                                  │
  │◄─────────── CMD ─────────────────│
  │                                  │
  │──────────── ACK ────────────────►│
  │                                  │
  │──────────── DATA ───────────────►│
  │         (result)                 │
```

### 6. Payload Formats

#### 6.1 DATA Payload

```json
{
  "timestamp": <unix_timestamp>,
  "sensor_id": <string>,
  "value": <number>,
  "unit": <string>
}
```

#### 6.2 CMD Payload

```json
{
  "command": <string>,
  "parameters": {
    "key": "value"
  }
}
```

#### 6.3 ERR Payload

```json
{
  "error_code": <number>,
  "message": <string>
}
```

### 7. Error Handling

#### 7.1 Error Codes

| Code | Description |
|------|-------------|
| 0x01 | Invalid message format |
| 0x02 | CRC checksum mismatch |
| 0x03 | Unsupported message type |
| 0x04 | Timeout |
| 0x05 | Device not ready |
| 0x10 | Command execution failed |

#### 7.2 Retry Mechanism

- Maximum retries: 3
- Timeout: 5 seconds per attempt
- Exponential backoff: 1s, 2s, 4s

### 8. Transport Layer

The protocol is transport-agnostic and can run over:

- UART/Serial
- TCP/IP
- UDP
- CAN bus
- USB

### 9. Security Considerations

- Optional message encryption (AES-128)
- Authentication via challenge-response
- Message sequence validation to prevent replay attacks

### 10. Implementation Requirements

#### 10.1 Minimum Requirements

- CRC-16/CCITT implementation
- Message framing/deframing
- Basic error handling

#### 10.2 Recommended Features

- Message queuing
- Automatic reconnection
- Statistics/diagnostics

### 11. Future Extensions

- Protocol version negotiation
- Multi-device support
- Broadcast messages
- Priority-based message queuing
