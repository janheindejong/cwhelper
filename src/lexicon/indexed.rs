use std::collections::HashMap;

use itertools::Itertools;

use crate::lexicon::{Lexicon, character_normalization::CharacterNormalization};

pub struct TrieNode<U, T> {
    value: Option<T>,
    children: HashMap<U, TrieNode<U, T>>,
}

impl TrieNode<char, String> {
    fn new() -> Self {
        TrieNode {
            value: None,
            children: HashMap::new(),
        }
    }
}

pub struct IndexedLexicon {
    root: TrieNode<char, String>,
}

impl Lexicon for IndexedLexicon {
    fn from_words(words: Vec<String>) -> Self {
        let mut root = IndexedLexicon {
            root: TrieNode::new(),
        };
        for word in words {
            let mut node = &mut root.root;
            for mut c in word.chars() {
                c = c.normalize(); // Convert to lowercase and remove diacritics characters used for indexing
                node = node.children.entry(c).or_insert_with(TrieNode::new);
            }
            node.value = Some(word.to_string());
        }
        root
    }

    fn find_matches(&self, target: &str) -> Vec<String> {
        let target: String = target.normalize(); // Convert to lowercase and remove diacritics in target string to match against index
        let mut layer = vec![&self.root];
        // We move through the tree layer by layer
        for c in target.chars() {
            // For every layer, we build the next layer from the nodes of the previous layer
            let mut next_layer: Vec<&TrieNode<char, String>> = vec![];
            for node in layer.iter() {
                // If character is wildcard, the next layer is composed of all children of all nodes in the current layer
                if c == '*' {
                    next_layer.extend(node.children.values());
                } else if let Some(layer_node) = node.children.get(&c) {
                    next_layer.push(layer_node);
                }
            }
            layer = next_layer;
        }
        // We extract the values from all the nodes in the final layer
        layer
            .iter()
            .filter_map(|n| n.value.clone())
            .sorted()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indexed_lexicon() -> IndexedLexicon {
        IndexedLexicon::from_words(
            vec!["ab", "abc", "adc", "café", "Kensington"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }

    #[test]
    fn test_ab() {
        let lexicon = indexed_lexicon();
        assert_eq!(
            lexicon.root.children[&'a'].children[&'b'].value,
            Some("ab".to_string())
        )
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

    #[test]
    fn find_a_star_c_returns_abc_and_adc() {
        let lexicon = indexed_lexicon();
        let matches = lexicon.find_matches("a*c");
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], "abc");
        assert_eq!(matches[1], "adc");
    }

    #[test]
    fn dialectics_are_ignored() {
        let lexicon = indexed_lexicon();
        let matches = lexicon.find_matches("ç*fe");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "café");
    }

    #[test]
    fn capitals_are_ignored() {
        let lexicon = indexed_lexicon();
        let matches = lexicon.find_matches("kensingTON");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], "Kensington");
    }
}
