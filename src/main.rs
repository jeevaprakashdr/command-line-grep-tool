use std::{io::{BufReader, BufRead}, fs::File};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let inner = File::open(&args.path).expect("cannot open file");
    let reader = BufReader::new(inner);
    
    for line in reader.lines(){
        match line {
            Ok(str) if str.contains(&args.pattern)  => println!("{}", str),
            Ok(_) => (),
            Err(_) => (),
        }
    }
}
