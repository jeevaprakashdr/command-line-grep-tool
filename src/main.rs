use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = match std::fs::read_to_string(&args.path) {
        Ok(content) => content,
        Err(err) => return Err(err.into()),
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
    
    Ok(())
}
