// Simple integration tests that work with the refactored functions

use swizzy::{group_issues_by_file, parse_swiftlint_output};

#[test]
fn test_end_to_end_parsing_and_grouping() {
    let json_input = r#"[
        {
            "file": "/project/FileA.swift",
            "line": 10,
            "character": 5,
            "severity": "warning",
            "reason": "First warning",
            "rule_id": "rule1"
        },
        {
            "file": "/project/FileB.swift",
            "line": 20,
            "severity": "error",
            "reason": "First error",
            "rule_id": "rule2"
        },
        {
            "file": "/project/FileA.swift",
            "line": 15,
            "severity": "warning",
            "reason": "Second warning",
            "rule_id": "rule1"
        }
    ]"#;

    // Test full parsing workflow
    let issues = parse_swiftlint_output(json_input).expect("Failed to parse");
    assert_eq!(issues.len(), 3);

    // Test grouping functionality
    let grouped = group_issues_by_file(issues);
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped["/project/FileA.swift"].len(), 2);
    assert_eq!(grouped["/project/FileB.swift"].len(), 1);

    // Verify issue details
    let file_a_issues = &grouped["/project/FileA.swift"];
    assert_eq!(file_a_issues[0].line, Some(10));
    assert_eq!(file_a_issues[0].character, Some(5));
    assert_eq!(file_a_issues[1].line, Some(15));
    assert_eq!(file_a_issues[1].character, None);

    let file_b_issues = &grouped["/project/FileB.swift"];
    assert_eq!(file_b_issues[0].severity, "error");
    assert_eq!(file_b_issues[0].reason, "First error");
}

#[test]
fn test_real_world_swiftlint_format() {
    // Test with format similar to actual SwiftLint output
    let json_input = r#"[
        {
            "character": null,
            "file": "/Users/dev/MyApp/ContentView.swift",
            "line": 1,
            "reason": "File name should match a type or extension declared in the file (if any)",
            "rule_id": "file_name",
            "severity": "Warning",
            "type": "File Name"
        },
        {
            "character": 16,
            "file": "/Users/dev/MyApp/ViewModel.swift",
            "line": 26,
            "reason": "Variable name 'x' should be between 3 and 40 characters long",
            "rule_id": "identifier_name",
            "severity": "Warning",
            "type": "Identifier Name"
        }
    ]"#;

    let issues = parse_swiftlint_output(json_input).expect("Should parse real-world data");
    assert_eq!(issues.len(), 2);

    // Verify the 'type' field is ignored (not part of our struct)
    assert_eq!(issues[0].character, None);
    assert_eq!(issues[1].character, Some(16));

    let grouped = group_issues_by_file(issues);
    assert_eq!(grouped.len(), 2);

    // Check that files are properly sorted in BTreeMap
    let file_names: Vec<_> = grouped.keys().collect();
    assert!(file_names.contains(&&"/Users/dev/MyApp/ContentView.swift".to_string()));
    assert!(file_names.contains(&&"/Users/dev/MyApp/ViewModel.swift".to_string()));
}

#[test]
fn test_error_handling() {
    // Test invalid JSON
    let invalid_json = "this is not valid json";
    let result = parse_swiftlint_output(invalid_json);
    assert!(result.is_err());

    // Test empty input
    let empty_result = parse_swiftlint_output("");
    assert!(empty_result.is_ok());
    assert_eq!(empty_result.unwrap().len(), 0);

    // Test whitespace-only input
    let whitespace_result = parse_swiftlint_output("   \n\t  ");
    assert!(whitespace_result.is_ok());
    assert_eq!(whitespace_result.unwrap().len(), 0);
}

#[test]
fn test_edge_cases() {
    // Test with missing optional fields
    let json_input = r#"[
        {
            "file": "/test/minimal.swift",
            "severity": "error",
            "reason": "Minimal issue"
        }
    ]"#;

    let issues = parse_swiftlint_output(json_input).expect("Should parse minimal fields");
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].file, "/test/minimal.swift");
    assert_eq!(issues[0].line, None);
    assert_eq!(issues[0].character, None);
    assert_eq!(issues[0].rule_id, None);
    assert_eq!(issues[0].severity, "error");
    assert_eq!(issues[0].reason, "Minimal issue");
}

#[test]
fn test_large_dataset() {
    // Generate a larger dataset to test performance/memory
    let mut json_issues = Vec::new();
    for i in 0..100 {
        json_issues.push(format!(
            r#"{{
                "file": "/large/project/file{}.swift",
                "line": {},
                "severity": "{}",
                "reason": "Issue number {}"
            }}"#,
            i % 5, // Group into 5 files
            i + 1,
            if i % 2 == 0 { "warning" } else { "error" },
            i
        ));
    }
    let json_input = format!("[{}]", json_issues.join(","));

    let issues = parse_swiftlint_output(&json_input).expect("Should parse large dataset");
    assert_eq!(issues.len(), 100);

    let grouped = group_issues_by_file(issues);
    assert_eq!(grouped.len(), 5); // Should group into 5 files

    // Each file should have 20 issues (100/5)
    for (_, file_issues) in grouped {
        assert_eq!(file_issues.len(), 20);
    }
}

#[test]
fn test_unicode_handling() {
    let json_input = r#"[
        {
            "file": "/проект/файл.swift",
            "line": 1,
            "severity": "error",
            "reason": "Unicode test: 你好世界 🌍",
            "rule_id": "unicode_rule"
        }
    ]"#;

    let issues = parse_swiftlint_output(json_input).expect("Should handle Unicode");
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].file, "/проект/файл.swift");
    assert!(issues[0].reason.contains("你好世界"));
    assert!(issues[0].reason.contains("🌍"));

    let grouped = group_issues_by_file(issues);
    assert_eq!(grouped.len(), 1);
    assert!(grouped.contains_key("/проект/файл.swift"));
}
