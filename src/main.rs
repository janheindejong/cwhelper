use std::{
    fs::File, io::{BufRead, BufReader, stdin}, process::exit
};

use unicode_normalization::UnicodeNormalization;

fn main() {
    // Query the user for a target in the form 'app*l'
    let target = prompt_target();

    // Read all the words from a textfile
    let wordlist = match get_wordlist("wordlist.txt") {
        Ok(list) => list, 
        Err(err) => {
            eprint!("{err}");
            exit(1)
        }
    };

    // Check target against wordlist to find matches
    for word in wordlist {
        if target.would_match(&word) {
            println!("{word}")
        }
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
fn get_wordlist(filename: &str) -> Result<Vec<String>, &'static str> {
    let reader = match File::open(filename) {
        Err(err) => {
            eprintln!("Couldn't read database with words: {err}");
            return Err("{err}")
        }
        Ok(file) => BufReader::new(file),
    };

    Ok(
        reader
            .lines()
            .filter_map(|line| match line {
                Ok(line) => Some(line),
                Err(msg) => {
                    eprintln!("{msg}");
                    None
                }
            })
            .collect()
    )
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
