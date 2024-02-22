#![allow(dead_code)]

use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_map = HashMap::new();
    let mut t_map = HashMap::new();
    for c in s.chars() {
        *s_map.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *t_map.entry(c).or_insert(0) += 1;
    }

    t_map.eq(&s_map)
}

#[cfg(test)]
mod tests{
    use crate::p242_valid_anagram::is_anagram;

    #[test]
    fn test_one() {
        assert!(is_anagram(String::from("anagram"), String::from("nagaram")))
    }

    #[test]
    #[should_panic]
    fn test_two() {
        assert!(is_anagram(String::from("rat"), String::from("car")))
    }

    #[test]
    #[should_panic]
    fn test_three() {
        assert!(is_anagram(String::from("ab"), String::from("a")))
    }
}
