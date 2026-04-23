pub mod simple;
pub mod words;

pub trait Lexicon {
    fn from_words(words: Vec<String>) -> Self;

    fn find_matches(&self, target: &str) -> Vec<String>;
}
