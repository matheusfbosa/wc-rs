use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short = 'c', long)]
    bytes: bool,
    #[arg(short, long)]
    lines: bool,
    #[arg(short, long)]
    words: bool,
    #[arg(short = 'm', long)]
    chars: bool,
    file_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let file_path = cli.file_path;

    let content: String = fs::read_to_string(&file_path)
        .map_err(|err| {
            eprintln!("error reading file '{}': {}", file_path.display(), err);
            std::process::exit(1);
        })
        .unwrap();

    match (cli.bytes, cli.lines, cli.words, cli.chars) {
        (true, _, _, _) => print_count(content.len(), file_path),
        (_, true, _, _) => print_count(content.lines().count(), file_path),
        (_, _, true, _) => print_count(content.split_whitespace().count(), file_path),
        (_, _, _, true) => print_count(content.chars().count(), file_path),
        _ => {
            eprintln!("error: please specify an argument");
            std::process::exit(1);
        }
    }
}

fn print_count(count: usize, file_path: PathBuf) {
    println!("{} {}", count, file_path.display());
}
