use std::{io::stdin, path::PathBuf, process::exit};

use clap::Parser;

use cwhelper::Lexicon;

/// Tool to help you solve the NRC Econogram
#[derive(Parser)]
struct Args {
    /// Word to match against lexicon (e.g., c*fe or pat*tzaak); if not passed, user is prompted
    word: Option<String>,

    /// Optional path to lexicon file, where each line is a word
    #[arg(short, long, value_name = "FILE")]
    lexicon: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Get target in the form 'app*l'
    let target = match args.word {
        Some(word) => word,
        None => prompt_target(),
    };

    let lexicon = match &args.lexicon {
        // If lexicon is passed as argument, load from file
        Some(path) => match Lexicon::from_file(path) {
            Ok(lexicon) => lexicon,
            Err(err) => {
                eprintln!("Couldn't read lexicon: {err}");
                exit(1)
            }
        },
        // By default, use Dutch lexicon
        None => Lexicon::dutch(),
    };

    let possible_matches = lexicon.find_matches(&target);

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
