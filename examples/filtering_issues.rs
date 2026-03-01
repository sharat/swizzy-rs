//! Filtering and analysis example for the swizzy library
//!
//! This example demonstrates how to filter and analyze SwiftLint issues
//! based on different criteria using swizzy's parsing capabilities.

use std::collections::HashMap;
use swizzy::{SwiftlintIssue, parse_swiftlint_output};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_input = r#"[
        {
            "file": "/path/to/ContentView.swift",
            "line": 12,
            "severity": "warning",
            "reason": "Line should be 120 characters or less",
            "rule_id": "line_length"
        },
        {
            "file": "/path/to/ContentView.swift",
            "line": 15,
            "severity": "error",
            "reason": "Missing documentation",
            "rule_id": "missing_docs"
        },
        {
            "file": "/path/to/Models/User.swift",
            "line": 8,
            "severity": "warning",
            "reason": "Variable name should be lowerCamelCase",
            "rule_id": "identifier_name"
        },
        {
            "file": "/path/to/Utils/Helper.swift",
            "line": 22,
            "severity": "error",
            "reason": "Force unwrapping should be avoided",
            "rule_id": "force_unwrapping"
        },
        {
            "file": "/path/to/Utils/Helper.swift",
            "line": 45,
            "severity": "warning",
            "reason": "Line should be 120 characters or less",
            "rule_id": "line_length"
        }
    ]"#;

    let issues = parse_swiftlint_output(json_input)?;

    println!("=== SwiftLint Issue Analysis ===\n");

    // Filter by severity
    let errors: Vec<&SwiftlintIssue> = issues
        .iter()
        .filter(|issue| issue.severity == "error")
        .collect();
    let warnings: Vec<&SwiftlintIssue> = issues
        .iter()
        .filter(|issue| issue.severity == "warning")
        .collect();

    println!("📊 Issues by Severity:");
    println!("  Errors: {}", errors.len());
    println!("  Warnings: {}\n", warnings.len());

    // Analyze by rule
    let mut rule_counts: HashMap<String, usize> = HashMap::new();
    for issue in &issues {
        if let Some(rule_id) = &issue.rule_id {
            *rule_counts.entry(rule_id.clone()).or_insert(0) += 1;
        }
    }

    println!("📋 Most Common Rules:");
    let mut rule_vec: Vec<_> = rule_counts.iter().collect();
    rule_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (rule, count) in rule_vec {
        println!("  {}: {} occurrences", rule, count);
    }

    // Find files with most issues
    let mut file_counts: HashMap<String, usize> = HashMap::new();
    for issue in &issues {
        *file_counts.entry(issue.file.clone()).or_insert(0) += 1;
    }

    println!("\n📁 Files with Most Issues:");
    let mut file_vec: Vec<_> = file_counts.iter().collect();
    file_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (file, count) in file_vec.iter().take(3) {
        println!(
            "  {}: {} issues",
            file.split('/').last().unwrap_or(file),
            count
        );
    }

    // Show only critical errors (for this example, force_unwrapping and missing_docs)
    println!("\n🚨 Critical Issues:");
    let critical_rules = ["force_unwrapping", "missing_docs"];
    let critical_issues: Vec<&SwiftlintIssue> = issues
        .iter()
        .filter(|issue| {
            issue
                .rule_id
                .as_ref()
                .map(|rule| critical_rules.contains(&rule.as_str()))
                .unwrap_or(false)
        })
        .collect();

    if critical_issues.is_empty() {
        println!("  No critical issues found! ✅");
    } else {
        for issue in critical_issues {
            println!(
                "  {} (line {}): {}",
                issue.file.split('/').last().unwrap_or(&issue.file),
                issue.line.unwrap_or(0),
                issue.reason
            );
        }
    }

    println!("\n✨ Analysis Complete!");

    Ok(())
}
