#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(is_word: bool) -> Self {
        Self {
            is_word,
            children: HashMap::new(),
        }
    }

    fn add_word(&mut self, word: &[char]) {
        let mut cur = self;
        for c in word {
            cur = cur.children.entry(*c).or_insert(TrieNode::new(false));
        }
        cur.is_word = true;
    }

    fn contains_word(&self, word: &[char]) -> bool {
        let mut cur = self;
        for c in word {
            cur = match cur.children.get(c) {
                Some(node) => node,
                None => { return false; }
            };
        }
        cur.is_word
    }
}

fn build_trie(words: Vec<String>) -> HashMap<char, TrieNode> {
    let mut result = HashMap::new();
    for word in words {
        let word = word.chars().collect::<Vec<char>>();
        let entry = result.entry(word[0]).or_insert(TrieNode::new(false));
        entry.add_word(&word[1..])
    }
    result
}


pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let trie = build_trie(word_dict);
    let mut dp = vec![false; s.len() + 1];
    dp[0] = true;

    for i in 1..s.len() + 1 {
        for j in 0..i {
            let substring = &s[j..i].chars().collect::<Vec<char>>();

            if let Some(node) = trie.get(&substring[0]) {
                if dp[j] && node.contains_word(&substring[1..]) {
                    dp[i] = true;
                }
            }
        }
    }

    dp[s.len()]
}


#[cfg(test)]
mod tests {
    use crate::p139_word_break::word_break;

    #[test]
    fn test_one() {
        assert_eq!(word_break(String::from("leetcode"), vec!["leet".to_string(), "code".to_string()]), true);
    }

    #[test]
    fn test_two() {
        assert_eq!(word_break(String::from("applepenapple"), vec!["apple".to_string(), "pen".to_string()]), true);
    }

    #[test]
    fn test_three() {
        assert_eq!(word_break(String::from("catsandog"), vec!["cats".to_string(), "dogs".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
    }

    #[test]
    fn test_four() {
        assert_eq!(word_break(String::from("aaaaaaa"), vec!["aaaa".to_string(), "aa".to_string()]), false);
    }

    #[test]
    fn test_five() {
        assert_eq!(word_break(String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
                              vec!["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"].iter().map(|s| s.to_string()).collect(),
        ), false);
    }
}