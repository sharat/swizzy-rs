use anyhow::{Context, Result};
use atty::Stream;
use colored::*;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    io::{self, Read},
    process,
};

#[derive(Debug, Deserialize)]
struct SwiftlintIssue {
    file: String,
    line: Option<usize>,
    character: Option<usize>,
    severity: String,
    reason: String,
    rule_id: Option<String>,
}

fn main() -> Result<()> {
    let _ = clap::App::new("swizzy")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Formats SwiftLint JSON output")
        .get_matches();

    let input = if atty::is(Stream::Stdin) {
        run_swiftlint()?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    if input.trim().is_empty() {
        return Ok(());
    }

    let issues: Vec<SwiftlintIssue> = serde_json::from_str(&input)
        .context("Failed to parse SwiftLint JSON output")?;

    if issues.is_empty() {
        return Ok(());
    }

    let mut output = String::new();
    let mut total = 0;

    let mut grouped = BTreeMap::new();
    for issue in issues {
        grouped.entry(issue.file.clone()).or_insert_with(Vec::new).push(issue);
    }

    for (file, issues) in grouped {
        output.push_str(&format!("{}\n", file.underline()));
        total += issues.len();

        for issue in issues {
            let line = issue.line.unwrap_or(0);
            let col = issue.character.unwrap_or(0);
            let severity = match issue.severity.as_str() {
                "Warning" => "warning".yellow(),
                _ => "error".red(),
            };

            // Format that VS Code can parse as clickable links
            output.push_str(&format!(
                "  {} {}:{}  {}  {}\n",
                " ".dimmed(),
                line.to_string().dimmed(),
                col.to_string().dimmed(),
                severity,
                issue.reason.trim_end_matches('.')
            ));

            if let Some(rule) = issue.rule_id {
                output.push_str(&format!("     {} {}\n", "rule:".dimmed(), rule.dimmed()));
            }
        }
        output.push('\n');
    }

    output.push_str(&format!(
        "{} {} problem{}\n",
        "✖".red().bold(),
        total,
        if total > 1 { "s" } else { "" }
    ));

    print!("{}", output);

    process::exit(1)
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

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
