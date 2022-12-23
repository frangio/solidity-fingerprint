use sha3::{Keccak256, Digest};
use solang_parser::lexer::Lexer;
use std::fs;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let contents = fs::read_to_string(&cli.file).unwrap();
    let mut comments = Vec::new();
    let mut errors = Vec::new();
    let lexer = Lexer::new(&contents, 0, &mut comments, &mut errors);
    let tokens = lexer
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .map(|(_, token, _)| token)
        .collect::<Vec<_>>();
    let mut hasher = Keccak256::new();
    hasher.update(format!("{:?}", tokens));
    let hash = hasher.finalize();
    println!("{:} {:016x}", cli.file.display(), hash);
}
