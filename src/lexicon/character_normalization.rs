use unicode_normalization::UnicodeNormalization;

pub trait CharacterNormalization<I> {
    /// Removes accents etc from string and converts to lowercase, e.g. turning Café into cafe
    fn normalize(&self) -> I;
}

impl CharacterNormalization<String> for str {
    fn normalize(&self) -> String {
        self.to_ascii_lowercase()
            .nfd()
            .filter(|c| c.is_ascii())
            .collect()
    }
}

impl CharacterNormalization<char> for char {
    fn normalize(&self) -> char {
        self.to_ascii_lowercase().nfd().next().unwrap()
    }
}
