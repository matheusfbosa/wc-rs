use std::fs;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    count_bytes: String,
}

fn main() {
    let cli = Cli::parse();
    let file_path: String = cli.count_bytes;

    let content: String = fs::read_to_string(file_path.clone()).unwrap_or_else(|err| {
        eprintln!("error reading file '{}': {}", file_path, err);
        std::process::exit(1);
    });
    let bytes_count = content.len();

    println!("{} {}", bytes_count, file_path);
}
