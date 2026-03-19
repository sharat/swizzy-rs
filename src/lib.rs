//! # swizzy
//!
//! A fast, developer-friendly SwiftLint output formatter that groups issues by file
//! and provides clickable file paths for editor integration.
//!
//! ## Features
//!
//! - **Grouped Output**: Issues are organized by file for better readability
//! - **Editor Integration**: File paths with line numbers are clickable in most editors/IDEs
//! - **Smart Colors**: Automatically disables colors when output is redirected
//! - **Zero Configuration**: Works out of the box with SwiftLint JSON output
//! - **Fast Performance**: Built with Rust for maximum speed
//! - **Flexible Input**: Accepts piped input or runs SwiftLint directly
//!
//! ## Usage
//!
//! ```bash
//! # Pipe SwiftLint JSON output to swizzy
//! swiftlint lint --reporter json | swizzy
//!
//! # Run from a directory containing Swift source files
//! swizzy
//! ```
//!
//! ## Exit Codes
//!
//! - `0`: No issues found or successfully processed
//! - `1`: Issues were found and reported
//!
//! ## Library Usage
//!
//! While swizzy is primarily a command-line tool, its core functionality
//! is available as a library for integration into other Rust applications:
//!
//! ```rust,no_run
//! use swizzy::{parse_swiftlint_output, group_issues_by_file, format_issues_output};
//!
//! let json_input = r#"[{"file": "test.swift", "line": 1, "severity": "warning", "reason": "Test issue", "rule_id": null}]"#;
//! let issues = parse_swiftlint_output(json_input).unwrap();
//! let grouped = group_issues_by_file(issues);
//! let (formatted, count) = format_issues_output(grouped, false);
//! println!("{}", formatted);
//! ```

use anyhow::{Context, Result};
use colored::{ColoredString, Colorize};
use serde::Deserialize;
use std::{collections::BTreeMap, fmt::Write};

/// Default line number when none is specified in the issue
const DEFAULT_LINE_NUMBER: usize = 1;
/// Warning severity string constant
const SEVERITY_WARNING: &str = "warning";
/// Error severity string constant
const SEVERITY_ERROR: &str = "error";

/// Represents a single SwiftLint issue parsed from JSON output.
///
/// This struct maps directly to the JSON format produced by SwiftLint's
/// `--reporter json` option. Each field corresponds to a property in the
/// SwiftLint JSON output.
///
/// # Examples
///
/// ```rust
/// use serde_json;
/// use swizzy::SwiftlintIssue;
///
/// let json = r#"{
///     "file": "/path/to/file.swift",
///     "line": 42,
///     "character": 10,
///     "severity": "warning",
///     "reason": "Line should be 120 characters or less",
///     "rule_id": "line_length"
/// }"#;
///
/// let issue: SwiftlintIssue = serde_json::from_str(json).unwrap();
/// assert_eq!(issue.file, "/path/to/file.swift");
/// assert_eq!(issue.line, Some(42));
/// assert_eq!(issue.severity, "warning");
/// ```
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct SwiftlintIssue {
    /// The file path where the issue was found
    pub file: String,
    /// The line number where the issue occurs (1-indexed, optional)
    pub line: Option<usize>,
    /// The character/column position where the issue occurs (optional)
    pub character: Option<usize>,
    /// The severity level of the issue (e.g., "warning", "error")
    pub severity: String,
    /// The human-readable description of the issue
    pub reason: String,
    /// The SwiftLint rule identifier that triggered this issue (optional)
    pub rule_id: Option<String>,
}

/// Parses SwiftLint JSON output into a vector of issues.
///
/// Takes the raw JSON output from SwiftLint's `--reporter json` option
/// and deserializes it into a vector of `SwiftlintIssue` structs.
///
/// # Arguments
///
/// * `input` - The JSON string output from SwiftLint
///
/// # Returns
///
/// * `Ok(Vec<SwiftlintIssue>)` - Successfully parsed issues
/// * `Err(anyhow::Error)` - If the JSON is malformed or doesn't match expected structure
///
/// # Examples
///
/// ```rust
/// use swizzy::parse_swiftlint_output;
///
/// let json_input = r#"[
///     {
///         "file": "/path/to/file.swift",
///         "line": 42,
///         "severity": "warning",
///         "reason": "Line too long",
///         "rule_id": "line_length"
///     }
/// ]"#;
///
/// let issues = parse_swiftlint_output(json_input).unwrap();
/// assert_eq!(issues.len(), 1);
/// assert_eq!(issues[0].file, "/path/to/file.swift");
/// ```
pub fn parse_swiftlint_output(input: &str) -> Result<Vec<SwiftlintIssue>> {
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(input).context("Failed to parse SwiftLint JSON output")
}

/// Groups SwiftLint issues by their file path.
///
/// Takes a vector of issues and organizes them into a map where each key
/// is a file path and each value is a vector of issues found in that file.
/// Uses `BTreeMap` to ensure consistent, sorted output.
///
/// # Arguments
///
/// * `issues` - Vector of SwiftLint issues to group
///
/// # Returns
///
/// A `BTreeMap` where keys are file paths and values are vectors of issues
/// for each file, sorted alphabetically by file path.
///
/// # Examples
///
/// ```rust
/// use swizzy::{SwiftlintIssue, group_issues_by_file};
///
/// let issues = vec![
///     SwiftlintIssue {
///         file: "FileA.swift".to_string(),
///         line: Some(1),
///         character: None,
///         severity: "warning".to_string(),
///         reason: "Issue 1".to_string(),
///         rule_id: None,
///     },
///     SwiftlintIssue {
///         file: "FileA.swift".to_string(),
///         line: Some(2),
///         character: None,
///         severity: "error".to_string(),
///         reason: "Issue 2".to_string(),
///         rule_id: None,
///     },
/// ];
///
/// let grouped = group_issues_by_file(issues);
/// assert_eq!(grouped.len(), 1);
/// assert_eq!(grouped.get("FileA.swift").unwrap().len(), 2);
/// ```
pub fn group_issues_by_file(issues: Vec<SwiftlintIssue>) -> BTreeMap<String, Vec<SwiftlintIssue>> {
    let mut grouped: BTreeMap<String, Vec<SwiftlintIssue>> = BTreeMap::new();
    for issue in issues {
        grouped
            .entry(issue.file.clone())
            .or_default()
            .push(issue);
    }
    grouped
}

/// Formats a single SwiftLint issue for display.
///
/// Converts a SwiftLint issue into a human-readable string format with
/// optional color coding. The format includes file path, line/column information,
/// severity level, issue description, and optional rule ID.
///
/// # Arguments
///
/// * `issue` - The SwiftLint issue to format
/// * `file` - The file path (used for clickable editor links)
/// * `use_colors` - Whether to apply terminal colors to the output
///
/// # Returns
///
/// A formatted string representation of the issue, ready for display.
///
/// # Examples
///
/// ```rust
/// use swizzy::{SwiftlintIssue, format_issue};
///
/// let issue = SwiftlintIssue {
///     file: "test.swift".to_string(),
///     line: Some(42),
///     character: Some(10),
///     severity: "warning".to_string(),
///     reason: "Line too long".to_string(),
///     rule_id: Some("line_length".to_string()),
/// };
///
/// let formatted = format_issue(&issue, "test.swift", false);
/// assert!(formatted.contains("test.swift:42:10"));
/// assert!(formatted.contains("warning"));
/// assert!(formatted.contains("Line too long"));
/// ```
pub fn format_issue(issue: &SwiftlintIssue, file: &str, use_colors: bool) -> String {
    let line = issue.line.unwrap_or(DEFAULT_LINE_NUMBER);
    let is_warning = issue.severity.to_lowercase() == SEVERITY_WARNING;
    let severity = if use_colors {
        if is_warning { SEVERITY_WARNING.yellow() } else { SEVERITY_ERROR.red() }
    } else {
        ColoredString::from(if is_warning { SEVERITY_WARNING } else { SEVERITY_ERROR })
    };

    // Include file path per line for clickable links in editors
    let loc = match issue.character {
        Some(c) => format!("{line}:{c}"),
        None => format!("{line}"),
    };

    let dim = |s: &str| -> ColoredString {
        if use_colors { s.dimmed() } else { ColoredString::from(s) }
    };

    let mut output = format!(
        "  {} {}:{}  {}  {}\n",
        dim(" "),
        dim(file),
        dim(&loc),
        severity,
        issue.reason.trim_end_matches('.')
    );

    if let Some(rule) = &issue.rule_id {
        output.push_str(&format!("     {} {}\n", dim("rule:"), dim(rule)));
    }

    output
}

/// Formats all grouped SwiftLint issues for display.
///
/// Takes a map of grouped issues and produces a formatted output string
/// with file headers, individual issue formatting, and a summary count.
/// This is the main formatting function that produces swizzy's characteristic
/// grouped output format.
///
/// # Arguments
///
/// * `grouped_issues` - Issues grouped by file path (from `group_issues_by_file`)
/// * `use_colors` - Whether to apply terminal colors and styling
///
/// # Returns
///
/// A tuple containing:
/// * `String` - The complete formatted output ready for printing
/// * `usize` - Total count of issues found
///
/// # Examples
///
/// ```rust
/// use std::collections::BTreeMap;
/// use swizzy::{SwiftlintIssue, format_issues_output};
///
/// let mut grouped = BTreeMap::new();
/// grouped.insert("test.swift".to_string(), vec![
///     SwiftlintIssue {
///         file: "test.swift".to_string(),
///         line: Some(1),
///         character: None,
///         severity: "warning".to_string(),
///         reason: "Test issue".to_string(),
///         rule_id: None,
///     }
/// ]);
///
/// let (output, count) = format_issues_output(grouped, false);
/// assert_eq!(count, 1);
/// assert!(output.contains("test.swift"));
/// assert!(output.contains("✖ 1 problem"));
/// ```
pub fn format_issues_output(
    grouped_issues: BTreeMap<String, Vec<SwiftlintIssue>>,
    use_colors: bool,
) -> (String, usize) {
    let mut output = String::new();
    let mut total = 0;

    for (file, issues) in grouped_issues {
        if use_colors {
            writeln!(output, "{}", file.underline()).unwrap();
        } else {
            writeln!(output, "{file}").unwrap();
        }
        total += issues.len();

        for issue in issues {
            output.push_str(&format_issue(&issue, &file, use_colors));
        }
        output.push('\n');
    }

    let plural = if total > 1 { "s" } else { "" };
    if use_colors {
        writeln!(output, "{} {total} problem{plural}", "✖".red().bold()).unwrap();
    } else {
        writeln!(output, "✖ {total} problem{plural}").unwrap();
    }
    (output, total)
}
