use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short('c'), long)]
    bytes: bool,
    #[arg(short, long)]
    lines: bool,
    #[arg(short, long)]
    words: bool,
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

    match (cli.bytes, cli.lines, cli.words) {
        (true, _, _) => {
            let bytes_count = content.len();
            println!("{} {}", bytes_count, file_path.display());
        }
        (_, true, _) => {
            let lines_count = content.lines().count();
            println!("{} {}", lines_count, file_path.display());
        }
        (_, _, true) => {
            let words_count = content.split_whitespace().count();
            println!("{} {}", words_count, file_path.display());
        }
        _ => {
            eprintln!("error: please specify an argument");
            std::process::exit(1);
        }
    }
}
