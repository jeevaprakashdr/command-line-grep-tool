use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Cli::parse();
    let file = File::open(&args.path).with_context(|| format!("could not find/open file: {}", args.path.display()))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(str) if str.contains(&args.pattern) => println!("{}", str),
            Ok(_) => (),
            Err(_) => (),
        }
    }

    Ok(())
}
