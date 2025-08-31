# swizzy-rs 🛠️

A Rust implementation of [swizzy](https://github.com/sharat/swizzy) output formatter into a developer-friendly format

![Example Output](https://github.com/sharat/swizzy-rs/raw/main/output.png)

## Installation 📦

### Pre-built Binary (Recommended)
```bash
cargo install --git https://github.com/sharat/swizzy-rs
```

# Usage

```
swiftlint lint --reporter json | swizzy
# or
swizzy # run from a directory containing swift source files
```

Notes:
- Exit code is 1 when any problems are found; otherwise 0.
- Colors are disabled when stdout is not a TTY.
- Each issue line includes file:line[:col] to enable clickable links in editors.

# Options

```
# Show version
swizzy --version

# Help information
swizzy --help
```