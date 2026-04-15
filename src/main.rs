use std::{
    fs::File,
    io::{self, BufRead, BufReader, stdin},
    path::PathBuf,
    process::exit,
};

use clap::Parser;

use unicode_normalization::UnicodeNormalization;

/// Tool to help you solve the NRC Econogram
#[derive(Parser)]
struct Args {
    /// Word to match against lexicon (e.g., c*fe or pat*tzaak)
    word: Option<String>,

    /// Path to lexicon file, where each line is a word
    #[arg(short, long, value_name = "FILE", default_value = "wordlist.txt")]
    lexicon: PathBuf,
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

    let possible_matches = lexicon.iter().filter(|word| target.would_match(&word));

    for word in possible_matches {
        println!("{word}")
    }
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
fn load_lexicon(filename: &PathBuf) -> Result<Vec<String>, io::Error> {
    let reader = BufReader::new(File::open(filename)?);

    let wordlist = reader
        .lines()
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            Err(msg) => {
                eprintln!("{msg}");
                None
            }
        })
        .collect();

    Ok(wordlist)
}

trait StrExt {
    /// Removes accents etc from string, e.g. turning café into cafe
    fn strip_diacritics(&self) -> String;

    /// Checks match; e.g., c*fe would match café, but carpool would not match car
    fn would_match(&self, other: &str) -> bool;
}

impl StrExt for str {
    fn strip_diacritics(&self) -> String {
        self.nfd().filter(|c| c.is_ascii()).collect()
    }

    fn would_match(&self, other: &str) -> bool {
        // Prep the target & reference to make sure we also match things like é and E to e
        let target = self.to_ascii_lowercase().strip_diacritics();
        let reference = other.to_ascii_lowercase().strip_diacritics();

        // Check if same length
        if target.chars().count() != reference.chars().count() {
            return false;
        }

        // Once we have determined that the length is the same,
        // check the pattern
        for (t, r) in target.chars().zip(reference.chars()) {
            if t == '*' {
                continue;
            }

            // When we detect a character that a character doesn't match, this
            // means the target and reference don't match
            if t != r {
                return false;
            }
        }

        true
    }
}
