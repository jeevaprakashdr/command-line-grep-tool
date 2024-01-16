use std::{string, fmt::format, result};

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("Count not read file '{}'", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}

fn find_matches(content:&str, pattern: &str, mut writer: impl std::io::Write){
    for line in content.lines() {
        if line.contains(&pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn test_find_a_match() {
    let mut result = Vec::new();
    find_matches("little known place has a lot to give", "has", &mut result);
    assert_eq!(result, b"little known place has a lot to give\n");
}