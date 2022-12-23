use solang_parser::lexer::Lexer;
use std::hash::{Hash, Hasher};
use std::{fs, collections::hash_map::DefaultHasher};
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
    let mut s = DefaultHasher::new();
    format!("{:?}", tokens).hash(&mut s);
    let h = s.finish();
    println!("{:} {:016x}", cli.file.display(), h);
}
