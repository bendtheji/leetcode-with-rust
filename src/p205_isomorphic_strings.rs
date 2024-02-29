#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut used = HashSet::new();
    let mut map = HashMap::new();

    for (c1, c2) in s.chars().zip(t.chars()) {
        match map.get(&c1) {
            Some(&c) => {
                if c != c2 {
                    return false;
                }
            }
            None => {
                if used.contains(&c2) { return false; }
                used.insert(c2);
                map.insert(c1, c2);
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::p205_isomorphic_strings::is_isomorphic;

    #[test]
    fn test_one() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
    }

    #[test]
    fn test_three() {
        assert!(is_isomorphic("paper".to_string(), "title".to_string()));
    }
}