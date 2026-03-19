# swizzy

[![Crates.io](https://img.shields.io/crates/v/swizzy.svg)](https://crates.io/crates/swizzy)
[![Documentation](https://docs.rs/swizzy/badge.svg)](https://docs.rs/swizzy)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast, developer-friendly SwiftLint output formatter that groups issues by file and provides clickable file paths for your editor.

![Example Output](https://github.com/sharat/swizzy-rs/raw/main/output.png)

## Features

- **Grouped Output**: Issues are organized by file for better readability
- **Editor Integration**: File paths with line numbers are clickable in most editors/IDEs
- **Smart Colors**: Automatically disables colors when output is redirected
- **Zero Configuration**: Works out of the box with SwiftLint JSON output
- **Fast Performance**: Built with Rust for maximum speed
- **Flexible Input**: Accepts piped input or runs SwiftLint directly

## Installation

### From crates.io (Recommended)
```bash
cargo install swizzy
```

### From Source
```bash
git clone https://github.com/sharat/swizzy-rs.git
cd swizzy-rs
cargo install --path .
```

## Usage

### Basic Usage
```bash
# Pipe SwiftLint JSON output to swizzy
swiftlint lint --reporter json | swizzy

# Run from a directory containing Swift source files
swizzy
```

### Integration Examples

#### With Xcode Build Phases
Add a new "Run Script Phase" in Xcode:
```bash
if which swizzy >/dev/null; then
    swiftlint lint --reporter json | swizzy
else
    echo "warning: swizzy not installed, run: cargo install swizzy"
fi
```

#### With GitHub Actions
```yaml
- name: Run SwiftLint with swizzy
  run: |
    swiftlint lint --reporter json | swizzy
```

#### With pre-commit hooks
```yaml
- repo: local
  hooks:
    - id: swiftlint-swizzy
      name: SwiftLint (formatted)
      entry: bash -c 'swiftlint lint --reporter json | swizzy'
      language: system
      types: [swift]
```

## Output Format

swizzy transforms SwiftLint's JSON output into a clean, grouped format:

```
src/ContentView.swift
   src/ContentView.swift:12:5  warning  Line should be 120 characters or less: currently 142 characters
     rule: line_length
   src/ContentView.swift:15:1  error    Missing documentation for public declaration
     rule: missing_docs

src/Models/User.swift
   src/Models/User.swift:8:10  warning  Variable name should be lowerCamelCase: 'user_id' should be 'userId'
     rule: identifier_name

✖ 3 problems
```

## Command Line Options

```bash
swizzy --version    # Show version information
swizzy --help       # Show help information
```

## Exit Codes

- **0**: No issues found or successfully processed
- **1**: Issues were found and reported

## Requirements

- SwiftLint (for direct execution mode)
- Rust 1.85+ (for building from source; required by Rust 2024 edition)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
```bash
git clone https://github.com/sharat/swizzy-rs.git
cd swizzy-rs
cargo build
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original [swizzy](https://github.com/sharat/swizzy) JavaScript implementation
- Built for the Swift development community
