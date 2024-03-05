#![allow(dead_code)]

pub fn minimum_length(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let mut left = 0 as i32;
    let mut right = (s.len() - 1) as i32;

    while left < (s.len() - 1) as i32 && right > 0 && s[left as usize] == s[right as usize] && left < right {
        while left < (s.len() - 1) as i32 && s[left as usize] == s[(left + 1) as usize] {
            left += 1;
        }

        while right > 0 && s[right as usize] == s[(right - 1) as usize] {
            right -= 1;
        }
        left += 1;
        right -= 1;
    }
    if left > right { 0 } else { (right - left + 1) as i32 }
}

#[cfg(test)]
mod tests {
    use crate::p1750_min_length_of_string_after_deleting_similar_ends::minimum_length;

    #[test]
    fn test_one() {
        let expected = 2;
        let output = minimum_length("ca".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = 0;
        let output = minimum_length("cabaabac".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let expected = 3;
        let output = minimum_length("aabccabba".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_four() {
        let expected = 1;
        let output = minimum_length("bbbbbbbbbbbbbbbbbbbbbbbbbbbabbbbbbbbbbbbbbbccbcbcbccbbabbb".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_five() {
        let expected = 0;
        let output = minimum_length("bbbbbbbbbbbbbbbbbbb".to_string());
        assert_eq!(expected, output);
    }
}