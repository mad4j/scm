# Contributing to SCM

Thank you for your interest in contributing to the Simple Communication Middleware (SCM) project!

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Coding Guidelines](#coding-guidelines)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)

## Getting Started

1. Read the [Protocol Specification](docs/protocol/specification.md)
2. Review the [Software Architecture](docs/architecture/software-components.md)
3. Check the [Issue Hierarchy](docs/issues/issue-hierarchy.md) for planned work
4. Look for issues labeled `good-first-issue` or `help-wanted`

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Git
- A text editor or IDE (VS Code with rust-analyzer recommended)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/mad4j/scm.git
cd scm

# Build the project
cargo build

# Run tests
cargo test

# Run clippy for linting
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## Project Structure

```
scm/
├── docs/                      # Documentation
│   ├── protocol/              # Protocol specification
│   ├── architecture/          # Software architecture
│   └── issues/                # Project planning
├── src/                       # Source code
│   ├── scm-core/              # Core protocol
│   ├── scm-device/            # Device implementation
│   └── scm-controller/        # Controller implementation
├── examples/                  # Example applications
└── tests/                     # Integration tests
```

## Development Workflow

### 1. Choose an Issue

- Browse the [issue hierarchy](docs/issues/issue-hierarchy.md)
- Check GitHub issues for available tasks
- Comment on the issue to claim it

### 2. Create a Branch

```bash
# Create a feature branch
git checkout -b feature/issue-number-description

# Or for bug fixes
git checkout -b fix/issue-number-description
```

### 3. Make Changes

- Follow the coding guidelines below
- Write tests for new functionality
- Update documentation as needed
- Commit changes with clear messages

### 4. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

### 5. Submit Pull Request

- Push your branch to GitHub
- Create a pull request
- Fill in the PR template
- Wait for review

## Coding Guidelines

### Rust Style

Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common mistakes
- Maximum line length: 100 characters
- Use 4 spaces for indentation

### Naming Conventions

- **Types**: `PascalCase` (e.g., `MessageType`, `DeviceClient`)
- **Functions**: `snake_case` (e.g., `send_message`, `calculate_crc`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `STX`, `MIN_FRAME_SIZE`)
- **Modules**: `snake_case` (e.g., `message`, `frame`)

### Code Organization

- Keep functions small and focused
- Use modules to organize related functionality
- Put public API at the top of modules
- Document all public items

### Error Handling

- Use `Result<T, Error>` for operations that can fail
- Create specific error types using `thiserror`
- Provide helpful error messages
- Don't use `unwrap()` or `expect()` in library code

### Comments and Documentation

```rust
/// Brief description of the function.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `param1` - Description of param1
/// * `param2` - Description of param2
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of possible errors
///
/// # Example
///
/// ```
/// use scm_core::Message;
/// let msg = Message::ping(1);
/// ```
pub fn function_name(param1: Type1, param2: Type2) -> Result<ReturnType> {
    // Implementation
}
```

## Testing

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_async_something() {
        // Async test code
    }
}
```

### Integration Tests

Place integration tests in the `tests/` directory:

```rust
// tests/integration_test.rs
use scm_core::*;

#[test]
fn test_end_to_end() {
    // Integration test
}
```

### Test Coverage

- Aim for >80% code coverage
- Test both success and error cases
- Test edge cases and boundary conditions
- Use property-based testing where appropriate

## Documentation

### Code Documentation

- Document all public APIs
- Include examples in documentation
- Explain why, not just what
- Keep documentation up to date

### User Documentation

- Update relevant documentation in `docs/`
- Add examples for new features
- Update README.md if needed
- Include migration guides for breaking changes

### Documentation Generation

```bash
# Generate and view documentation
cargo doc --no-deps --open

# Check documentation
cargo doc --no-deps
```

## Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] All tests pass
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] Commit messages are clear
- [ ] Branch is up to date with main

### PR Template

```markdown
## Description
Brief description of the changes

## Related Issue
Closes #issue-number

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] All tests pass
```

### Review Process

1. Automated checks must pass (formatting, tests, clippy)
2. At least one maintainer approval required
3. Address review comments
4. Maintainer will merge when ready

## Issue Labels

- `good-first-issue` - Good for newcomers
- `help-wanted` - Extra attention needed
- `bug` - Bug reports and fixes
- `enhancement` - New features
- `documentation` - Documentation improvements
- `epic:*` - Epic tracking labels

## Communication

- GitHub Issues - Bug reports and feature requests
- Pull Requests - Code contributions
- Discussions - General questions and ideas

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## Questions?

If you have questions:
1. Check existing documentation
2. Search closed issues
3. Open a new issue with your question

Thank you for contributing to SCM!
