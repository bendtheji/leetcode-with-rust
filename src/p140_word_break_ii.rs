#![allow(dead_code)]

use std::collections::HashMap;

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let root = form_trie(word_dict);
    let mut result = vec![];

    let mut curr = String::from("");

    backtracking(&s, 0, 0, &mut curr, &mut result, &root);
    result
}

fn backtracking(s: &str, start_index: usize, end_index: usize, curr_str: &mut String, result: &mut Vec<String>, trie: &TrieNode) {
    if end_index == s.len() && is_word(&s[start_index..end_index], trie) {
        curr_str.push_str(&s[start_index..end_index]);
        result.push(curr_str.clone());
        curr_str.replace_range(curr_str.len() - (end_index - start_index).., "");
        return;
    }

    if end_index == s.len() {
        return;
    }

    if is_word(&s[start_index..end_index], trie) {
        curr_str.push_str(&s[start_index..end_index]);
        curr_str.push_str(" ");
        backtracking(s, end_index, end_index, curr_str, result, trie);
        curr_str.replace_range(curr_str.len() - (end_index - start_index + 1).., "");
    }

    backtracking(s, start_index, end_index + 1, curr_str, result, trie);
}

#[derive(Debug)]
struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(is_word: bool, children: HashMap<char, TrieNode>) -> Self {
        Self { is_word, children }
    }
}

fn form_trie(word_dict: Vec<String>) -> TrieNode {
    let mut root = TrieNode::new(false, HashMap::new());
    for word in word_dict {
        let mut curr = &mut root;
        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(TrieNode::new(false, HashMap::new()));
        }
        curr.is_word = true;
    }
    root
}

fn is_word(word: &str, trie: &TrieNode) -> bool {
    let mut curr = trie;
    for c in word.chars() {
        match curr.children.get(&c) {
            Some(child) => { curr = child }
            None => return false,
        }
    }
    curr.is_word
}

#[cfg(test)]
mod tests {
    use crate::p140_word_break_ii::word_break;

    #[test]
    fn test_one() {
        let s = String::from("catsanddog");
        let word_dict = vec![String::from("cat"),
                             String::from("cats"),
                             String::from("and"),
                             String::from("sand"),
                             String::from("dog")];
        let expected = vec![String::from("cat sand dog"), String::from("cats and dog")];
        let output = word_break(s, word_dict);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = String::from("pineapplepenapple");
        let word_dict = vec![String::from("apple"),
                             String::from("pen"),
                             String::from("applepen"),
                             String::from("pine"),
                             String::from("pineapple")];
        let expected = vec![String::from("pine apple pen apple"),
                            String::from("pine applepen apple"),
                            String::from("pineapple pen apple")];
        let output = word_break(s, word_dict);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let s = String::from("catsandog");
        let word_dict = vec![String::from("cat"),
                             String::from("cats"),
                             String::from("and"),
                             String::from("sand"),
                             String::from("dog")];
        let expected: Vec<String> = vec![];
        let output = word_break(s, word_dict);
        assert_eq!(expected, output);
    }
}