pub mod simple;
pub mod words;

pub trait FindMatches {
    fn find_matches(&self, target: &str) -> Vec<String>;
}
