//! Basic usage example for the swizzy library
//!
//! This example demonstrates how to use swizzy as a library to parse
//! and format SwiftLint JSON output programmatically.

use swizzy::{format_issues_output, group_issues_by_file, parse_swiftlint_output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample SwiftLint JSON output
    let json_input = r#"[
        {
            "file": "/path/to/ContentView.swift",
            "line": 12,
            "character": 5,
            "severity": "warning",
            "reason": "Line should be 120 characters or less: currently 142 characters",
            "rule_id": "line_length"
        },
        {
            "file": "/path/to/ContentView.swift",
            "line": 15,
            "character": 1,
            "severity": "error",
            "reason": "Missing documentation for public declaration",
            "rule_id": "missing_docs"
        },
        {
            "file": "/path/to/Models/User.swift",
            "line": 8,
            "character": 10,
            "severity": "warning",
            "reason": "Variable name should be lowerCamelCase: 'user_id' should be 'userId'",
            "rule_id": "identifier_name"
        }
    ]"#;

    // Parse the JSON output
    println!("Parsing SwiftLint JSON output...");
    let issues = parse_swiftlint_output(json_input)?;
    println!("Found {} issues", issues.len());

    // Group issues by file
    println!("\nGrouping issues by file...");
    let grouped_issues = group_issues_by_file(issues);
    println!("Issues found in {} files", grouped_issues.len());

    // Format for display (without colors for this example)
    println!("\nFormatted output:");
    let (formatted_output, total_issues) = format_issues_output(grouped_issues, false);
    print!("{formatted_output}");

    println!("Total issues processed: {total_issues}");

    Ok(())
}
