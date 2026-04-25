use std::{
    io::{self, Write, stdin, stdout},
    path::PathBuf,
    process::exit,
};

use clap::{Parser, ValueEnum};
use cwhelper::lexicon::{Lexicon, simple::SimpleLexicon, words};
use thiserror::Error;

#[derive(Clone, ValueEnum, Copy)]
enum Language {
    English,
    Dutch,
    Italian,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Couldn't load lexicon from {path:?}: {error}")]
    LexiconError { path: PathBuf, error: io::Error },
    #[error("Error writing output {0}")]
    WriteError(io::Error),
}

impl Language {
    fn words(&self) -> Vec<String> {
        match self {
            Language::Dutch => words::dutch(),
            Language::English => words::english(),
            Language::Italian => words::italian(),
        }
    }
}

/// Tool to help you solve the NRC Econogram
#[derive(Parser)]
struct Args {
    /// Word to match against lexicon (e.g., c*fe or pat*tzaak); if not passed, user is prompted
    word: Option<String>,

    /// Optional path to lexicon file, where each line is a word
    #[arg(long, value_name = "FILE")]
    lexicon_file: Option<PathBuf>,

    // Select language to use
    #[arg(short, long, value_enum)]
    language: Option<Language>,
}

fn main() {
    let args = Args::parse();
    let mut writer = stdout();
    if let Err(e) = run(&args, &mut writer) {
        eprintln!("Error: {e}");
        exit(1)
    };
}

fn run(args: &Args, writer: &mut impl Write) -> Result<(), AppError> {
    let lexicon = build_lexicon(args)?;
    let target = args.word.clone().unwrap_or_else(|| prompt_target());
    for word in lexicon.find_matches(&target) {
        writeln!(writer, "{word}").map_err(AppError::WriteError)?;
    }
    Ok(())
}

fn build_lexicon(args: &Args) -> Result<SimpleLexicon, AppError> {
    let words = match &args.lexicon_file {
        // If lexicon is passed as argument, load from file
        Some(path) => words::from_file(path).map_err(|e| AppError::LexiconError {
            path: path.clone(),
            error: e,
        })?,
        // Else, use built-in
        None => args.language.unwrap_or(Language::English).words(),
    };
    let lexicon = SimpleLexicon::from_words(words);
    Ok(lexicon)
}

/// Gets the target string from the CLI
fn prompt_target() -> String {
    let mut input = String::new();
    loop {
        input.clear();
        println!("Enter the word you want to search for, using * for missing characters: ");
        if stdin().read_line(&mut input).is_err() {
            println!("Try again...");
            continue;
        }
        break input.trim().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let args = Args {
            word: Some("t*st".to_string()),
            lexicon_file: None,
            language: None,
        };
        let mut writer = Vec::new();
        let res = run(&args, &mut writer);
        let output = String::from_utf8(writer).unwrap();
        assert!(res.is_ok());
        assert_eq!(output, "Test\ntost\nTrst\nTSST\n");
    }

    #[test]
    fn test_run_incorrect_lexicon_path_returns_error() {
        let args = Args {
            word: Some("t*st".to_string()),
            lexicon_file: Some(PathBuf::from("non-existent-path")),
            language: None,
        };
        let mut writer = Vec::new();
        let res = run(&args, &mut writer);
        assert!(res.is_err());
        match res {
            Err(AppError::LexiconError { path, error: _ }) => {
                assert_eq!(path, PathBuf::from("non-existent-path"))
            }
            _ => panic!("expected LexiconError"),
        }
    }
}
