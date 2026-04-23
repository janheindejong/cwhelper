use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub struct Words {}

impl Words {
    pub fn english() -> Vec<String> {
        Words::split_string(include_str!("words/english.txt"))
    }

    pub fn dutch() -> Vec<String> {
        Words::split_string(include_str!("words/dutch.txt"))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dutch_lexicon_has_413938_words() {
        assert_eq!(Words::dutch().len(), 413937);
    }

    #[test]
    fn english_lexicon_has_413938_words() {
        assert_eq!(Words::english().len(), 466550);
    }
}
