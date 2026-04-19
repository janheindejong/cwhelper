use unicode_normalization::UnicodeNormalization;

pub struct Lexicon {
    words: Vec<String>,
}

impl Lexicon {
    pub fn new(words: Vec<String>) -> Self {
        Lexicon { words }
    }

    pub fn find_matches(&self, target: &str) -> Vec<&String> {
        self.words
            .iter()
            .filter(|word| target.would_match(word))
            .collect()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn lexicon() -> Lexicon {
        let words = vec!["café", "carpool", "carport", "brick", "carpenter", "Carter"];
        let words = words.iter().map(|w| w.to_string()).collect();
        Lexicon { words }
    }

    #[test]
    fn target_should_match() {
        let lexicon = lexicon();
        let res = lexicon.find_matches("car****");
        assert_eq!(res.len(), 2)
    }

    #[test]
    fn diacritics_should_be_ignored() {
        let lexicon = lexicon();
        let res = lexicon.find_matches("c*fe");
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn capitals_should_be_ignored() {
        let lexicon = lexicon();
        let res = lexicon.find_matches("c*rtEr");
        assert_eq!(res.len(), 1);
    }
}
