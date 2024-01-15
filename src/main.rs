use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no command is provided");
    let path = std::env::args().nth(2).expect("no path is given");

    let args = Cli{
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("{:?}", args);
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}
