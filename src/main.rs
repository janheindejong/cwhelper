use std::{io::{self, stdin, stdout, Write}, path::PathBuf, process::exit
};

use clap::{Parser, ValueEnum};
use cwhelper::lexicon::{Lexicon, simple::SimpleLexicon, words};

#[derive(Clone, ValueEnum, Copy)]
enum Language {
    English,
    Dutch,
    Italian,
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
    let res = run(args, &mut writer);
    if let Err(msg) = res {
        println!("{}", msg);
        exit(1)
    };
}

fn run(args: Args, writer: &mut impl Write) -> io::Result<()>{
    let (target, lexicon) = parse_args(&args)?;

    for word in lexicon.find_matches(&target) {
        writeln!(writer, "{word}")?;
    }
    Ok(())
}

fn parse_args(args: &Args) -> Result<(String, SimpleLexicon), io::Error> {
    let target = args.word.clone().unwrap_or_else(|| prompt_target());
    let lexicon = build_lexicon(args)?;
    Ok((target, lexicon))
}

fn build_lexicon(args: &Args) -> Result<SimpleLexicon, io::Error> {
    let words = match &args.lexicon_file {
        // If lexicon is passed as argument, load from file
        Some(path) => words::from_file(path)?,
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
            language: None
        };
        let mut writer: Vec<u8> = Vec::new();
        let res = run(args, &mut writer);
        let output = String::from_utf8(writer).unwrap();
        assert!(res.is_ok());
        assert_eq!(output, "Test\ntost\nTrst\nTSST\n");
    }
}
