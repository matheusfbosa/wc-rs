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
        (true, _, _, _) => print_count(count_bytes(&content), file_path),
        (_, true, _, _) => print_count(count_lines(&content), file_path),
        (_, _, true, _) => print_count(count_words(&content), file_path),
        (_, _, _, true) => print_count(count_chars(&content), file_path),
        _ => {
            println!(
                "{}\t{}\t{}\t{}",
                count_lines(&content),
                count_words(&content),
                count_bytes(&content),
                file_path.display()
            );
        }
    }
}

fn count_bytes(content: &str) -> usize {
    content.len()
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

fn count_chars(content: &str) -> usize {
    content.chars().count()
}

fn print_count(count: usize, file_path: PathBuf) {
    println!("{}\t{}", count, file_path.display());
}
