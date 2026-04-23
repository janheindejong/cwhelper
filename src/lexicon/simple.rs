use std::{io, path::PathBuf};

use super::FindMatches;
use super::words::Words;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

#[derive(PartialEq, Deserialize, Eq, Hash, Debug)]
pub struct SimpleLexicon {
    words: Vec<String>,
}

impl SimpleLexicon {
    pub fn from_words(words: Vec<String>) -> Self {
        SimpleLexicon { words }
    }

    pub fn dutch() -> Self {
        SimpleLexicon {
            words: Words::dutch(),
        }
    }

    pub fn english() -> Self {
        SimpleLexicon {
            words: Words::english(),
        }
    }

    pub fn from_file(filename: &PathBuf) -> Result<Self, io::Error> {
        let words = Words::from_file(filename)?;
        Ok(SimpleLexicon::from_words(words))
    }
}

impl FindMatches for SimpleLexicon {
    fn find_matches(&self, target: &str) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| target.would_match(word))
            .map(|x| x.clone())
            .collect()
    }
}

trait StringCleaning {
    /// Removes accents etc from string, e.g. turning café into cafe
    fn strip_diacritics(&self) -> String;
}

impl StringCleaning for str {
    fn strip_diacritics(&self) -> String {
        self.nfd().filter(|c| c.is_ascii()).collect()
    }
}

trait StringMatching {
    /// Checks match; e.g., c*fe would match café, but carpool would not match car
    fn would_match(&self, other: &str) -> bool;
}

impl StringMatching for str {
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

    fn simple_lexicon() -> SimpleLexicon {
        let words = vec!["café", "carpool", "carport", "brick", "carpenter", "Carter"];
        let words = words.iter().map(|w| w.to_string()).collect();
        SimpleLexicon::from_words(words)
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
    fn english_lexicon_has_2_as_first_word() {
        let lexicon = SimpleLexicon::english();
        assert_eq!(lexicon.words[0], "2")
    }

    #[test]
    fn dutch_lexicon_has_010_as_first_word() {
        let lexicon = SimpleLexicon::dutch();
        assert_eq!(lexicon.words[0], "010")
    }
}
