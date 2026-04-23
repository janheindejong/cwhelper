use std::collections::HashMap;

pub struct TrieNode<U, T> {
    value: Option<T>, 
    children: HashMap<U, TrieNode<U, T>>
}

impl TrieNode<char, String> {
    fn new() -> Self {
        TrieNode { value: None, children: HashMap::new() }
    }
}

pub struct IndexedLexicon<U, T> {
    root: TrieNode<U, T>
}

impl IndexedLexicon<char, String> {
    pub fn from_words(words: Vec<&str>) -> Self {
        let mut root = IndexedLexicon { root: TrieNode::new() };
        for word in words {
            let mut node = &mut root.root;
            for c in word.chars() {
                node = node.children.entry(c).or_insert_with(TrieNode::new);
            }
            node.value = Some(word.to_string());
        }
        root
    }

    pub fn find_matches(&self, target: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in target.chars() {
            node = match node.children.get(&c) {
                Some(node) => node, 
                None => return vec![]
            };
        }
        match node.value.as_ref() {
            Some(res) => vec![res.to_string()],
            None => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indexed_lexicon() -> IndexedLexicon<char, String> {
        IndexedLexicon::from_words(vec!["ab", "abc"])
    }

    #[test]
    fn test_ab() {
        let lexicon = indexed_lexicon();
        assert_eq!(lexicon.root.children[&'a'].children[&'b'].value, Some("ab".to_string()))
    }

    #[test]
    fn find_ab_returns_ab() {
        let lexicon = indexed_lexicon();
        let matches = lexicon.find_matches("ab");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "ab");
    }

    #[test]
    fn find_a_returns_empty_vector() {
        let lexicon = indexed_lexicon();
        let matches = lexicon.find_matches("a");
        assert_eq!(matches.len(), 0)
    }

    #[test]
    fn find_a_star_returns_ab() {
        let lexicon = indexed_lexicon();
        let matches = lexicon.find_matches("a*");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "ab");
    }

}