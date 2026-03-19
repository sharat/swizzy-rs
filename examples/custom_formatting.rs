//! Custom formatting example for the swizzy library
//!
//! This example shows how to create custom formatting logic
//! using swizzy's parsing capabilities.

use swizzy::{group_issues_by_file, parse_swiftlint_output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_input = r#"[
        {
            "file": "/path/to/file1.swift",
            "line": 10,
            "severity": "error",
            "reason": "Force casting should be avoided",
            "rule_id": "force_cast"
        },
        {
            "file": "/path/to/file1.swift",
            "line": 20,
            "severity": "warning",
            "reason": "Line too long",
            "rule_id": "line_length"
        },
        {
            "file": "/path/to/file2.swift",
            "line": 5,
            "severity": "error",
            "reason": "Force unwrapping should be avoided",
            "rule_id": "force_unwrapping"
        }
    ]"#;

    let issues = parse_swiftlint_output(json_input)?;
    let grouped_issues = group_issues_by_file(issues);

    // Custom formatting: Summary by severity
    let mut error_count = 0;
    let mut warning_count = 0;

    println!("=== SwiftLint Issues Summary ===\n");

    for (file, file_issues) in &grouped_issues {
        println!("📁 {file}");

        for issue in file_issues {
            match issue.severity.as_str() {
                "error" => {
                    println!(
                        "  🔴 Error (line {}): {}",
                        issue.line.unwrap_or(0),
                        issue.reason
                    );
                    error_count += 1;
                }
                "warning" => {
                    println!(
                        "  🟡 Warning (line {}): {}",
                        issue.line.unwrap_or(0),
                        issue.reason
                    );
                    warning_count += 1;
                }
                _ => {
                    println!(
                        "  ⚪ Other (line {}): {}",
                        issue.line.unwrap_or(0),
                        issue.reason
                    );
                }
            }

            if let Some(rule_id) = &issue.rule_id {
                println!("     Rule: {rule_id}");
            }
        }
        println!();
    }

    println!("=== Final Summary ===");
    println!("🔴 Errors: {error_count}");
    println!("🟡 Warnings: {warning_count}");
    println!("📁 Files affected: {}", grouped_issues.len());
    println!("📊 Total issues: {}", error_count + warning_count);

    Ok(())
}
