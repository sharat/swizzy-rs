use anyhow::{Context, Result};
use colored::control;
use std::{
    io::{self, IsTerminal, Read},
    process,
};
use swizzy::{format_issues_output, group_issues_by_file, parse_swiftlint_output};

/// Exit code when SwiftLint issues are found
const EXIT_CODE_WITH_ISSUES: i32 = 1;

fn main() -> Result<()> {
    let _ = clap::Command::new("swizzy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Formats SwiftLint JSON output")
        .get_matches();

    // Enable colors when stdout is a TTY
    let use_colors = io::stdout().is_terminal();
    control::set_override(use_colors);

    let input = if io::stdin().is_terminal() {
        run_swiftlint()?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let issues = parse_swiftlint_output(&input)?;

    if issues.is_empty() {
        return Ok(());
    }

    let grouped_issues = group_issues_by_file(issues);
    let (output, total) = format_issues_output(grouped_issues, use_colors);

    print!("{output}");

    if total > 0 {
        process::exit(EXIT_CODE_WITH_ISSUES);
    }

    Ok(())
}

fn run_swiftlint() -> Result<String> {
    eprintln!("No input piped. Running `swiftlint lint --reporter json`...");

    let output = std::process::Command::new("swiftlint")
        .args(["lint", "--reporter", "json"])
        .output()
        .context("Failed to execute swiftlint. Is it installed and in your PATH?")?;

    if !output.status.success() {
        eprintln!(
            "SwiftLint exited with error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use swizzy::{SwiftlintIssue, group_issues_by_file, parse_swiftlint_output};

    #[test]
    fn test_parse_swiftlint_output_empty() {
        let result = parse_swiftlint_output("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<SwiftlintIssue>::new());
    }

    #[test]
    fn test_parse_swiftlint_output_valid_json() {
        let json_input = r#"[
            {
                "file": "/test/file.swift",
                "line": 10,
                "character": 5,
                "severity": "warning",
                "reason": "Test warning",
                "rule_id": "test_rule"
            }
        ]"#;

        let result = parse_swiftlint_output(json_input);
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].file, "/test/file.swift");
        assert_eq!(issues[0].line, Some(10));
        assert_eq!(issues[0].severity, "warning");
    }

    #[test]
    fn test_group_issues_by_file_multiple_files() {
        let issue1 = SwiftlintIssue {
            file: "/file1.swift".to_string(),
            line: Some(1),
            character: Some(1),
            severity: "warning".to_string(),
            reason: "Test".to_string(),
            rule_id: None,
        };
        let mut issue2 = issue1.clone();
        let mut issue3 = issue1.clone();

        issue2.file = "/file2.swift".to_string();
        issue3.file = "/file1.swift".to_string();

        let issues = vec![issue1, issue2, issue3];
        let grouped = group_issues_by_file(issues);

        assert_eq!(grouped.len(), 2);
        assert_eq!(grouped["/file1.swift"].len(), 2);
        assert_eq!(grouped["/file2.swift"].len(), 1);
    }

    #[test]
    fn test_integration_with_real_data() {
        let json_input = r#"[
            {
                "character": null,
                "file": "/Users/user/project/MyApp/Component.swift",
                "line": 1,
                "reason": "File name should match a type or extension declared in the file (if any)",
                "rule_id": "file_name",
                "severity": "Warning",
                "type": "File Name"
            }
        ]"#;

        let issues = parse_swiftlint_output(json_input).expect("Should parse real-world data");
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].character, None);
        assert_eq!(issues[0].severity, "Warning");

        let grouped = group_issues_by_file(issues);
        assert_eq!(grouped.len(), 1);
    }
}
