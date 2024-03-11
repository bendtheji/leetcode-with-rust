#![allow(dead_code)]

use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut map = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let mut result = vec![];

    for c in order.chars() {
        match map.get(&c) {
            Some(&count) => {
                for _ in 0..count {
                    result.push(c);
                }
            }
            _ => (),
        }
        map.remove(&c);
    }

    for entry in map {
        for _ in 0..entry.1 {
            result.push(entry.0);
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::p791_custom_sort_string::custom_sort_string;

    #[test]
    fn test_one() {
        let order = "cba".to_string();
        let s = "abcd".to_string();

        let expected = "cbad".to_string();
        let output = custom_sort_string(order, s);
        assert_eq!(expected, output);
    }
}