# AGENTS.md - Agentic Coding Guidelines for rsbt

## Overview

rsbt (Rust BitTorrent) is a Rust workspace project implementing a BitTorrent client.
The workspace contains multiple crates for different functionality including bencode parsing,
torrent handling, CLI, web interface, and more.

## Build Commands

### Full Build & Test
```bash
cargo build --verbose        # Build all workspace crates
cargo test --verbose        # Run all tests
```

### Single Crate Operations
```bash
cargo build -p <crate-name>     # Build specific crate
cargo test -p <crate-name>      # Test specific crate
```

### Running Single Tests
```bash
# Run a specific test by name
cargo test <test_function_name>

# Run tests in a specific file
cargo test --test <test_file_name>

# Run tests in a specific crate
cargo test -p rsbt-bencode-nom

# Run tests with output
cargo test -- --nocapture
```

### Documentation
```bash
cargo doc --no-deps          # Generate documentation
```

### Linting
```bash
cargo clippy                 # Run clippy lints
cargo clippy -- -D warnings  # Treat warnings as errors
```

### Formatting
```bash
cargo fmt                   # Format code (uses rustfmt defaults)
cargo fmt -- --check        # Check formatting without modifying
```

## Project Structure

```
rsbt/                    # Workspace root
├── rsbt/                # Main rsbt crate (contains bins, cli, web)
│   ├── bins/            # Binary targets (rsbt.rs, rsbt-cli.rs)
│   ├── cli/             # Command-line interface
│   └── web/             # Web functionality
├── rsbt-app/            # Core application logic
├── rsbt-types/          # Core types (no_std compatible)
├── rsbt-defs/           # Definitions
├── rsbt-bencode-nom/    # Bencode parsing (no_std compatible)
├── rsbt-bencode-derive/ # Derive macros for bencode
├── rsbt-memory-hub/     # Memory hub implementation
└── _rsbt/               # Legacy/alternative crate
```

## Code Style Guidelines

### General Conventions
- Minimum Rust version: 1.94.0 (specified in Cargo.toml)
- Edition: 2024
- Use stable Rust only
- Enable strict Rust features where possible

### Naming Conventions
- **Types**: `PascalCase` (e.g., `Torrent`, `DownloadCommand`)
- **Functions**: `snake_case` (e.g., `run()`, `calculate_sha1()`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `BITTORRENT_HANDSHAKE`)
- **Modules**: `snake_case` (e.g., `mod command;`)
- **Traits**: `PascalCase` (e.g., `Runnable`)
- **Enums**: `PascalCase` with variants in `PascalCase` or `SCREAMING_SNAKE_CASE`

### Imports
- Use absolute imports with `crate::` for internal modules
- Use external crate imports directly (e.g., `use tokio::...`)
- Group imports: standard library first, then external crates, then internal modules
- Use `mod` to declare submodules, `use` to bring items into scope

Example:
```rust
use std::path::PathBuf;
use rsbt_app::{AppError, DefaultRuntimeBuilder, Download};
use super::super::SomeParentModule;
mod command;
```

### Error Handling
- Use `thiserror` for error types (workspace dependency)
- Define errors as enums with `#[derive(Debug, thiserror::Error)]`
- Use `Result<T, E>` for fallible operations
- Propagate errors with `?` operator when possible
- Return appropriate error variants from `AppError` enum

Example pattern from codebase:
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Documentation
- Use doc comments `///` for public API items
- Include usage examples in documentation
- Document enum variants and struct fields
- Use markdown for formatting in docs

### Code Organization
- One module per file, declared in `lib.rs` or `mod.rs`
- Keep related functionality together
- Use feature flags for optional functionality (e.g., `no_std` support)
- Mark tests with `#[cfg(test)]` module

### Type System
- Prefer strong typing over raw types
- Use lifetime annotations explicitly where needed (`'a`)
- Use generics where code can be reused
- Enable `no_std` support in appropriate crates (`rsbt-types`, `rsbt-bencode-nom`)

### Testing
- Place unit tests in `#[cfg(test)]` modules within source files
- Place integration tests in `tests/` directory
- Use `include_bytes!` for test fixtures (see `_rsbt-bencode/tests/`)
- Follow standard test naming: `#[test]` functions with descriptive names

### Attributes and Derives
- Common derives: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`
- Parser/CLI: `#[derive(Debug, clap::Parser)]`
- Error types: `#[derive(Debug, thiserror::Error)]`
- Bencode parsing: `#[derive(Debug, BencodeParse)]` (custom derive)

### Async/Concurrency
- Uses `tokio` for async runtime (workspace dependency)
- Use `tokio-util` for codec and I/O utilities
- Use `tokio-stream` for stream abstractions
- Configure features carefully (see workspace Cargo.toml)

### Dependencies
- Manage dependencies centrally in workspace Cargo.toml
- Use workspace dependencies (`dependencies.workspace = true`)
- Carefully select features to minimize binary size
- Default features disabled where possible

### Logging & Tracing
- Uses `tracing` for structured logging (workspace dependency)
- Uses `tracing-subscriber` for output formatting
- Set up global subscriber in application entry points

## Environment Configuration

- Default config directory: `$HOME/.rsbt/`
- Default download directory: `$HOME/.rsbt/download/`
- Environment variables: Use `.env` file with `dotenv`
- Configuration file: `torrents.toml` in config directory

## CI/CD

- Uses GitHub Actions (`.github/workflows/rust.yml`)
- Tests run on: ubuntu-latest, windows-latest, macos-latest
- Build command: `cargo build --verbose`
- Test command: `cargo test --verbose`

### Memory Arena
- `rsbt-memory-hub` provides fixed-size bump allocator for constrained environments
- Designed for Raspberry Pi (~128MB budget)
- Uses `Vec::leak` pattern for static buffer
- Supports `no_std` with `alloc` feature
- Zero-copy integration with bencode parsing

## Architecture Insights

### Runtime Choice
- Supports multiple runtime backends: tokio (async), lightweight alternatives, and completely blocking variant
- Avoid over-abstraction; keep generic bounds simple and minimal
- Consider runtime trait/abstraction when needed for flexibility

### Actor Model
- Uses actor-based concurrency for handling concurrent downloads
- Actors communicate via message channels (`tokio::sync::mpsc`)
- Pattern: spawn actor, get handle, send messages through channel
- Keep generic bounds simple and minimal (simplified in commit `484b5fa`)

### Bencode Parsing
- Custom `BencodeParse` derive macro for parsing BitTorrent bencode format
- Located in `rsbt-bencode-derive` crate
- Supports struct parsing, enum variants, custom converters
- Use `#[bencode(rename = "field")]` for field mapping

### no_std Design
- Core crates (`rsbt-types`, `rsbt-bencode-nom`) support `no_std`
- Use `#[cfg_attr(not(feature = "std"), no_std)]` attribute
- Provide `std` feature for optional std support

## Development Notes

- Main entry point for CLI: `rsbt/cli/src/lib.rs` (run function)
- Binary targets in `rsbt/bins/src/bin/`
- The workspace uses resolver version 2
- Release profile has LTO enabled (`lto = true`)