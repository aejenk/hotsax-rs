use std::collections::HashMap;

// TODO: Work on improving displaying the TRIE.
/// An augmented trie that works using recursion.
/// All leaf nodes in this trie must be `IndexNode`s, which represent the index the word is found in.
/// All words added to the trie must be of the **same size**.
#[derive(Debug)]
pub enum AugmentedTrie {
    /// This represents any node of the trie that isn't the final one.
    /// A hashmap that maps a character onto a *branch*.
    NestedNode(HashMap<char, AugmentedTrie>),

    /// This represents the penultimate leaves of the trie.
    /// Contains a list of indexes of this sequence from the array up to this point.
    IndexNode(Vec<usize>)
}

impl AugmentedTrie {
    /// Creates a new `AugmentedTrie` with the root node being nested.
    pub fn new() -> Self {
        AugmentedTrie::NestedNode(HashMap::new())
    }

    /// Adds a new word to the trie, appending the index given.
    pub fn add_word(&mut self, word: &str, index: usize) {
        if word.is_empty() {
            panic!("An empty string cannot be added into the trie.");
        }

        match self {
            AugmentedTrie::NestedNode(ref mut map) => {
                // If there's two or more characters, then another nested node should be added.
                if word.len() >= 2 {
                    let char = word.chars().nth(0).unwrap();
                    match map.get_mut(&char) {
                        Some(node) => node.add_word(&word[1..], index),
                        None => {
                            let mut trie = AugmentedTrie::new();
                            trie.add_word(&word[1..], index);
                            map.insert(
                                char,
                                trie
                            );
                        }
                    }
                }
                // If there's only 1 character, then add an IndexNode with the character, or
                // add the character to the IndexNode.
                else {
                    let char = word.chars().nth(0).unwrap();
                    match map.get_mut(&char) {
                        Some(AugmentedTrie::IndexNode(ref mut vec)) => {
                            vec.push(index);
                        },
                        None => {
                            map.insert(char, AugmentedTrie::IndexNode(vec![index]));
                        },
                        _ => panic!("All words need to be of constant size.")
                    }
                }
            },
            _ => panic!("Can't add a word to an IndexNode.")
        }
    }

    /// Generates a trie from a list of words, alongside their indexes.
    pub fn from_words(words: Vec<(usize, &String)>) -> Self {
        let mut trie = AugmentedTrie::new();
        words.into_iter().for_each(|(index, word)| trie.add_word(word, index));
        trie
    }

    /// Retrieves the indexes of a given word.
    pub fn get_indexes(&self, word: &str) -> &Vec<usize> {
        match self {
            AugmentedTrie::NestedNode(map) => {
                if word.len() >= 2 {
                    match map.get(&word.chars().nth(0).unwrap()) {
                        Some(map) =>  map.get_indexes(&word[1..]),
                        None => panic!("Word '{}' doesn't exist in trie.", word)
                    }
                } else {
                    match map.get(&word.chars().nth(0).unwrap()) {
                        Some(AugmentedTrie::IndexNode(indexes)) => indexes,
                        None => panic!("Word '{}' doesn't exist in trie.", word),
                        _ => panic!("Words have to be at the same length.")
                    }
                }
            },
            _ => panic!("An index node doesn't have any words.")
        }
    }
}