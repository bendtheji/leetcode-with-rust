#![allow(dead_code)]

use std::collections::HashSet;

pub fn min_remove_to_make_valid(s: &str) -> String {
    let mut rights = vec![];
    let mut stack = vec![];

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => {
                stack.push(i);
            }
            ')' => {
                if let Some(_) = stack.last() {
                    stack.pop();
                } else {
                    rights.push(i);
                }
            }
            _ => ()
        }
    }

    let set = stack.into_iter().chain(rights.into_iter()).collect::<HashSet<usize>>();

    let mut result = String::with_capacity(s.len());
    for (i, c) in s.chars().enumerate() {
        if !set.contains(&i) { result.push(c); }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::p1249_minimum_remove_to_make_valid_parentheses::min_remove_to_make_valid;

    #[test]
    fn test_one() {
        let s = "lee(t(c)o)de)";
        let expected = "lee(t(c)o)de";
        let output = min_remove_to_make_valid(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = "a)b(c)d";
        let expected = "ab(c)d";
        let output = min_remove_to_make_valid(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let s = "))((";
        let expected = "";
        let output = min_remove_to_make_valid(s);
        assert_eq!(expected, output);
    }
}