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

### Workflows
| Workflow | File | Purpose |
|----------|------|---------|
| Build | `.github/workflows/build.yml` | Format, lint, test, audit on push/PR |
| Release | `.github/workflows/release.yml` | Multi-platform binary release on git tag |
| Dependabot | `.github/dependabot.yml` | Weekly Friday 03:30 UTC dependency updates |

### Release Process

**Trigger:** Git tag push (e.g., `v1.2.0`)

```bash
# Bump version in Cargo.toml, commit, create tag, push
cd /Users/sarat/oss/swizzy-rs
# Edit Cargo.toml version manually
git add Cargo.toml
git commit -m "chore(release): bump version to 1.2.0"
git tag v1.2.0
git push origin main --follow-tags
```

**What happens:**
1. Tag push triggers `.github/workflows/release.yml`
2. Build: binaries for macOS Intel (x86_64) and Apple Silicon (aarch64)
3. Package: tar.gz archives
4. Create GitHub Release with auto-generated release notes
5. Attach binary artifacts

### Publishing to crates.io
```bash
cargo publish
```
- Requires `CARGO_REGISTRY_TOKEN` secret in GitHub (not yet configured ⚠️)

### Requirements
- `GITHUB_TOKEN` (auto-provided)
- `CARGO_REGISTRY_TOKEN` (for crates.io — needs to be configured)

## Notes
- Currently only builds for macOS targets (Intel + Apple Silicon)
- Uses semantic version tags: `v1.*.*`
- Multi-platform support can be added by extending the matrix in release.yml
