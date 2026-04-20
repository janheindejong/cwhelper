use std::{
    io::{self, stdin},
    path::PathBuf,
    process::exit,
};

use clap::{Parser, ValueEnum};

use cwhelper::Lexicon;

#[derive(Clone, ValueEnum)]
enum Language {
    English,
    Dutch,
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

    let (target, lexicon) = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        exit(1)
    });

    for word in lexicon.find_matches(&target) {
        println!("{word}")
    }
}

fn parse_args(args: &Args) -> Result<(String, Lexicon), io::Error> {
    let target = args.word.clone().unwrap_or_else(|| prompt_target());
    let lexicon = build_lexicon(args)?;
    Ok((target, lexicon))
}

fn build_lexicon(args: &Args) -> Result<Lexicon, io::Error> {
    let lexicon = match &args.lexicon_file {
        // If lexicon is passed as argument, load from file
        Some(path) => Lexicon::from_file(path)?,
        // Else, use built-in
        None => match &args.language {
            Some(language) => match language {
                Language::Dutch => Lexicon::dutch(),
                Language::English => Lexicon::english(),
            },
            // By default, use English
            None => Lexicon::english(),
        },
    };
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
