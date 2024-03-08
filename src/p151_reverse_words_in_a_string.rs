#![allow(dead_code)]

pub fn reverse_words(s: String) -> String {
    s.split_ascii_whitespace().rev().fold(String::new(), |mut acc, s| {
        acc.push_str(s);
        acc.push(' ');
        acc
    }).trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::p151_reverse_words_in_a_string::reverse_words;

    #[test]
    fn test_one() {
        let expected = "blue is sky the".to_string();
        let output = reverse_words("the sky is blue".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = "world hello".to_string();
        let output = reverse_words("  hello world  ".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let expected = "example good a".to_string();
        let output = reverse_words("a good   example".to_string());
        assert_eq!(expected, output);
    }
}