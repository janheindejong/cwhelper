use crate::lexicon::character_normalization::CharacterNormalization;

use super::Lexicon;
use serde::Deserialize;

#[derive(PartialEq, Deserialize, Eq, Hash, Debug)]
pub struct SimpleLexicon {
    words: Vec<String>,
}

impl Lexicon for SimpleLexicon {
    fn from_words(words: Vec<String>) -> Self {
        SimpleLexicon { words }
    }

    fn find_matches(&self, target: &str) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| target.would_match(word))
            .map(|x| x.clone())
            .collect()
    }
}

trait StringMatching {
    /// Checks match; e.g., c*fe would match café, but carpool would not match car
    fn would_match(&self, other: &str) -> bool;
}

impl StringMatching for str {
    fn would_match(&self, other: &str) -> bool {
        // Prep the target & reference to make sure we also match things like é and E to e
        let target = self.normalize();
        let reference = other.normalize();

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
        let words = ["café", "carpool", "carport", "brick", "carpenter", "Carter"];
        SimpleLexicon::from_words(words.map(String::from).to_vec())
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
}
