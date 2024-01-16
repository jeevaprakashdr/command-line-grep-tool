use std::{string, fmt::format};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), Box<CustomError>> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
    .map_err(|err| CustomError(format!("error reading '{}' : {}", args.path.display(), err)))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
    
    Ok(())
}
