# Getting Started with SCM

Welcome to the Simple Communication Middleware (SCM) project!

## What is SCM?

SCM is a lightweight, bidirectional communication protocol designed for communication between embedded devices and control nodes implementing Human-Machine Interface (HMI) functionality.

## Project Status

✅ **Foundation Complete** - Version 0.1.0

The project has been set up with:
- Complete protocol specification
- Detailed software architecture
- Reference implementation skeleton with core functionality
- Comprehensive documentation
- Project planning and issue hierarchy

## Quick Links

### Documentation
- 📖 [Protocol Specification](protocol/specification.md) - Complete protocol definition
- 🏗️ [Software Architecture](architecture/software-components.md) - Component design and structure
- 📋 [Issue Hierarchy](issues/issue-hierarchy.md) - Project planning and tracking

### Getting Started
- 📚 [Contributing Guidelines](../CONTRIBUTING.md) - How to contribute
- 💻 [Main README](../README.md) - Project overview

## What's Implemented

### Protocol Specification ✅
- Message frame structure (STX, LEN, TYPE, ID, PAYLOAD, CRC)
- Message types (PING, PONG, DATA, CMD, ACK, NACK, ERR)
- Communication flows
- Error handling and error codes
- Transport layer abstraction

### Software Architecture ✅
- Component layering (Application, API, Protocol Core, Transport)
- Module structure for three main crates
- Data flow diagrams
- State management design
- Configuration formats

### Reference Implementation 🔄

#### scm-core (Core Protocol) ✅
- ✅ Message structure (`Message`, `Payload`)
- ✅ Frame encoding/decoding (`Frame`)
- ✅ CRC-16/CCITT checksum implementation
- ✅ Protocol types and constants (`MessageType`, error codes)
- ✅ Error handling
- ✅ Unit tests (11 tests passing)

#### scm-device (Device Side) 🔄
- ✅ Basic device client structure (`DeviceClient`)
- ✅ Connection state management
- ✅ Message ID generation
- ⏳ Transport layer (planned)
- ⏳ Connection management (planned)
- ⏳ Command handler framework (planned)

#### scm-controller (Controller Side) 🔄
- ✅ Basic controller structure (`Controller`)
- ✅ Device registry
- ✅ Multi-device management
- ⏳ Transport layer (planned)
- ⏳ Command dispatch (planned)
- ⏳ Event system (planned)

## Next Steps

Based on the [Issue Hierarchy](issues/issue-hierarchy.md), the development roadmap is:

### Phase 1: Complete Core Implementation (Current)
Epic 3 - Issues 3.1 through 3.10
- ✅ 3.1: Project structure setup
- ✅ 3.2: Message data structures
- ✅ 3.3: CRC-16 implementation
- ✅ 3.4: Frame serialization
- ✅ 3.5: Frame deserialization
- ✅ 3.6: Protocol state machine (partial)
- ✅ 3.7: Message validation
- ✅ 3.8: Error handling
- ✅ 3.9: Unit tests
- ⏳ 3.10: Documentation and examples

### Phase 2: Device Implementation
Epic 4 - Issues 4.1 through 4.12
- Transport abstraction
- Connection management
- Command handlers
- Keep-alive mechanism

### Phase 3: Controller Implementation
Epic 5 - Issues 5.1 through 5.11
- Multi-device support
- Command dispatch
- Data collection
- Event notification

### Phase 4: Examples and Tools
Epic 6 - Issues 6.1 through 6.8
- Example applications
- Command-line tools
- Benchmarks

### Phase 5: Testing and Documentation
Epics 7 and 8
- Comprehensive testing
- Complete documentation
- Version 1.0 release

## How to Contribute

1. Read the [Contributing Guidelines](../CONTRIBUTING.md)
2. Check the [Issue Hierarchy](issues/issue-hierarchy.md) for available tasks
3. Look for issues labeled `good-first-issue`
4. Follow the development workflow

## Building and Testing

```bash
# Clone the repository
git clone https://github.com/mad4j/scm.git
cd scm

# Build all crates
cargo build

# Run tests
cargo test

# Build release version
cargo build --release

# Generate documentation
cargo doc --no-deps --open
```

## Project Structure

```
scm/
├── docs/                          # Documentation
│   ├── protocol/                  # Protocol specification
│   ├── architecture/              # Software architecture
│   └── issues/                    # Project planning
├── src/                           # Source code
│   ├── scm-core/                  # Core protocol
│   ├── scm-device/                # Device implementation
│   └── scm-controller/            # Controller implementation
├── examples/                      # Example applications
├── tests/                         # Integration tests
├── Cargo.toml                     # Workspace configuration
├── README.md                      # Project overview
├── CONTRIBUTING.md                # Development guidelines
└── LICENSE                        # MIT License
```

## Questions?

- 📖 Check the documentation in `docs/`
- 🐛 Open an issue on GitHub
- 💡 Read the [Contributing Guidelines](../CONTRIBUTING.md)

## License

MIT License - see [LICENSE](../LICENSE) file for details.

---

**Status Legend:**
- ✅ Complete
- 🔄 In Progress
- ⏳ Planned
- ❌ Blocked
