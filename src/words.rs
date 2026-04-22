use std::{fs::File, io::{self, BufReader, BufRead}, path::PathBuf};


pub struct Words {}

impl Words {
    pub fn english() -> Vec<String> {
        Words::split_string(include_str!("lexicons/english.txt"))
    }

    pub fn dutch() -> Vec<String> {
        Words::split_string(include_str!("lexicons/dutch.txt"))
    }

    pub fn from_file(filename: &PathBuf) -> Result<Vec<String>, io::Error> {
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

        Ok(words)
    }

    fn split_string(words: &str) -> Vec<String> {
        words.lines().map(|x| x.to_string()).collect()
    }
}
