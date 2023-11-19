use std::fs;
use std::io::{self, Read};
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
    file_path: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let (content, file_path) = read_content(&cli.file_path);

    match (cli.bytes, cli.lines, cli.words, cli.chars) {
        (true, _, _, _) => print_count(count_bytes(&content), file_path),
        (_, true, _, _) => print_count(count_lines(&content), file_path),
        (_, _, true, _) => print_count(count_words(&content), file_path),
        (_, _, _, true) => print_count(count_chars(&content), file_path),
        _ => println!(
            "{}\t{}\t{}\t{}",
            count_lines(&content),
            count_words(&content),
            count_bytes(&content),
            file_path.unwrap_or_default().display()
        ),
    }
}

fn read_content(file_path: &Option<PathBuf>) -> (String, Option<PathBuf>) {
    match file_path {
        Some(path) => {
            let content = fs::read_to_string(path)
                .map_err(|err| {
                    eprintln!("error reading file '{}': {}", path.display(), err);
                    std::process::exit(1);
                })
                .unwrap_or_else(|_| String::new());

            (content, Some(path.clone()))
        }
        None => {
            let mut buffer = String::new();
            if let Err(err) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("error reading from stdin: {}", err);
                std::process::exit(1);
            }

            (buffer, None)
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

fn print_count(count: usize, file_path: Option<PathBuf>) {
    match file_path {
        Some(path) => println!("{}\t{}", count, path.display()),
        None => println!("{}", count),
    }
}
