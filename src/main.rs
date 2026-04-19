use std::{
    fs::File,
    io::{self, BufRead, BufReader, stdin},
    path::PathBuf,
    process::exit,
};

use clap::Parser;

use econogram_helper::Lexicon;

/// Tool to help you solve the NRC Econogram
#[derive(Parser)]
struct Args {
    /// Word to match against lexicon (e.g., c*fe or pat*tzaak)
    word: Option<String>,

    /// Path to lexicon file, where each line is a word
    #[arg(short, long, value_name = "FILE", default_value = "wordlist.txt")]
    lexicon: PathBuf,
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

/// Extracts all the words from a *.txt file
fn load_lexicon(filename: &PathBuf) -> Result<Lexicon, io::Error> {
    let reader = BufReader::new(File::open(filename)?);

    let words = reader
        .lines()
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            Err(msg) => {
                eprintln!("{msg}");
                None
            }
        })
        .collect();

    Ok(Lexicon::new(words))
}

fn main() {
    let args = Args::parse();

    // Get target in the form 'app*l'
    let target = match args.word {
        Some(word) => word,
        None => prompt_target(),
    };

    // Read all the words from a textfile
    let lexicon = match load_lexicon(&args.lexicon) {
        Ok(list) => list,
        Err(err) => {
            eprintln!("{err}");
            exit(1)
        }
    };

    let possible_matches = lexicon.find_matches(&target);

    for word in possible_matches {
        println!("{word}")
    }
}
