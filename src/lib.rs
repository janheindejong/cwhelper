use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use unicode_normalization::UnicodeNormalization;

pub struct Lexicon {
    words: Vec<String>,
}

impl Lexicon {
    pub fn from_words(words: Vec<String>) -> Self {
        Lexicon { words }
    }

    pub fn dutch() -> Self {
        let words = include_str!("lexicons/dutch.txt")
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        Lexicon { words }
    }

    pub fn english() -> Self {
        let words = include_str!("lexicons/english.txt")
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        Lexicon { words }
    }

    pub fn from_file(filename: &PathBuf) -> Result<Self, io::Error> {
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

        Ok(Lexicon::from_words(words))
    }

    pub fn find_matches(&self, target: &str) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| target.would_match(word))
            .map(|x| x.clone())
            .collect()
    }
}

trait StringMatching {
    /// Removes accents etc from string, e.g. turning café into cafe
    fn strip_diacritics(&self) -> String;

    /// Checks match; e.g., c*fe would match café, but carpool would not match car
    fn would_match(&self, other: &str) -> bool;
}

impl StringMatching for str {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_lexicon() -> Lexicon {
        let words = vec!["café", "carpool", "carport", "brick", "carpenter", "Carter"];
        let words = words.iter().map(|w| w.to_string()).collect();
        Lexicon { words }
    }

    #[test]
    fn target_should_match() {
        let lexicon = simple_lexicon();
        let res = lexicon.find_matches("car****");
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn diacritics_should_be_ignored() {
        let lexicon = simple_lexicon();
        let res = lexicon.find_matches("c*fe");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "café");
    }

    #[test]
    fn capitals_should_be_ignored() {
        let lexicon = simple_lexicon();
        let res = lexicon.find_matches("c*rtEr");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], "Carter");
    }

    #[test]
    fn dutch_lexicon_has_413938_words() {
        let lexicon = Lexicon::dutch();
        assert_eq!(lexicon.words.len(), 413938);
    }

    #[test]
    fn dutch_actiecom_finds_2_matches() {
        let lexicon = Lexicon::dutch();
        let matches = lexicon.find_matches("actiecom***");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], "actiecomedy");
        assert_eq!(matches[1], "actiecomité");
    }

    #[test]
    fn english_lexicon_has_413938_words() {
        let lexicon = Lexicon::english();
        assert_eq!(lexicon.words.len(), 466551);
    }

    #[test]
    fn english_txst_finds_4_matches() {
        let lexicon = Lexicon::english();
        let matches = lexicon.find_matches("t*st");
        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0], "Test");
        assert_eq!(matches[1], "tost");
    }
}
