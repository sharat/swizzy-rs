# AGENTS.md

This file provides guidance for agents working in this repository.

## Build / Lint / Test Commands

```bash
# Build
cargo build                        # debug build
cargo build --release              # optimized build

# Testing
cargo test --verbose               # run all tests

# Formatting
cargo fmt -- --check               # check formatting
cargo fmt                          # apply formatting

# Linting
cargo clippy -- -D warnings       # lint with warnings-as-errors
```

## Project Overview

swizzy-rs is a fast, developer-friendly SwiftLint output formatter written in Rust. It groups issues by file and provides clickable file paths for editor integration.

### Key Features
- Parses SwiftLint JSON output
- Groups issues by file for readability
- Supports colored terminal output
- Clickable file paths (IDE integration)

## Dependency Management

```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo search <crate_name>
```

## CI/CD

Check `.github/workflows/` for CI configuration.
