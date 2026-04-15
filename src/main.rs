use std::{fs::File, io::{BufRead, BufReader, stdin}, process::exit};

use unicode_normalization::UnicodeNormalization;

fn main() {

    // Query the user for a target in the form 'app*l'
    let target = prompt_target();

    // Read all the words from a textfile
    let wordlist = get_wordlist("wordlist.txt");

    // Check target against wordlist to find matches
    for word in wordlist {
        if check_match(&target, &word) {
            println!("{word}")
        }
    }
}

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

fn get_wordlist(filename: &str) -> Vec<String> {
    let reader = match File::open(filename) {
        Err(msg) => {
            eprintln!("Couldn't read database with words: {msg}");
            exit(1)
        },
        Ok(file) => BufReader::new(file)
    };

    reader.lines()
        .map(|line| match line {
            Ok(line) => line, 
            Err(msg) => {
                eprintln!("{msg}");
                exit(1);
            }
        })
        .collect()
}

fn check_match(target: &str, reference: &str) -> bool {
    
    // Prep the target & reference to make sure we also match things like é and E to e
    let target = target.to_ascii_lowercase().strip_diacritics();
    let reference = reference.to_ascii_lowercase().strip_diacritics();

    // Check if same length
    if target.chars().count() != reference.chars().count() {
        return false
    }

    // Once we have determined that the length is the same, 
    // check the pattern
    let target = target.chars();
    let reference = reference.chars();
    
    for (t, r) in target.zip(reference) {
        if t == '*' {
            continue;
        }

        // When we detect a character that a character doesn't match, this 
        // means the target and reference don't match
        if t != r {
            return false
        }
    }

    true
}

trait StrExt {
    fn strip_diacritics(&self) -> String;
}

impl StrExt for str {
    fn strip_diacritics(&self) -> String {
        self.nfd().filter(|c| c.is_ascii()).collect()
    }
}
