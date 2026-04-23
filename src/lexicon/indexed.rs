use std::collections::HashMap;

use itertools::Itertools;

use crate::lexicon::{Lexicon, character_normalization::CharacterNormalization};

pub struct TrieNode {
    value: Option<String>,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            value: None,
            children: HashMap::new(),
        }
    }
}

pub struct IndexedLexicon {
    root: TrieNode,
}

impl Lexicon for IndexedLexicon {
    fn from_words(words: Vec<String>) -> Self {
        let mut lexicon = IndexedLexicon {
            root: TrieNode::new(),
        };
        for word in words {
            // Get the node at the end of the tree, that matches the pattern of the word
            let mut node = &mut lexicon.root;
            for c in word.chars() {
                // Get the node for this layer, or add an empty one
                node = node
                    .children
                    .entry(c.normalize()) // Use normalized character for indexing
                    .or_insert_with(TrieNode::new);
            }
            // Set the value of the node to the word
            node.value = Some(word.to_string());
        }
        lexicon
    }

    fn find_matches(&self, target: &str) -> Vec<String> {
        let target: String = target.normalize(); // Convert to lowercase and remove diacritics in target string to match against index
        // The algorithm moves through the tree in layers matching the characters of the target,
        // gathering all nodes that match the pattern up to that point.
        let mut layer = vec![&self.root];
        for c in target.chars() {
            // For every layer, we iterate over the nodes of the previous layer, and use them to
            // populate the next layer.
            let mut next_layer: Vec<&TrieNode> = vec![];
            for node in layer.iter() {
                // When a character is wildcard, all children of the node are added to the next layer
                if c == '*' {
                    next_layer.extend(node.children.values());
                }
                // Otherwise we try to see if the node has children for the given character, and add them to the layer.
                else if let Some(layer_node) = node.children.get(&c) {
                    next_layer.push(layer_node);
                }
            }
            layer = next_layer;
        }
        // Finally, we extract the values from all the nodes in the final layer and return them
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
        let words = ["ab", "abc", "adc", "café", "Kensington"];
        IndexedLexicon::from_words(words.map(String::from).to_vec())
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
