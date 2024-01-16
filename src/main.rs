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
    .with_context(|| format!("Could not read file '{}'", args.path.display()))?;

    command_line_grep_tool::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}