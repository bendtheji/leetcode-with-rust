#![allow(dead_code)]

pub fn length_of_last_word(s: String) -> i32 {
    let words: Vec<&str> = s.trim().split_ascii_whitespace().collect();
    words[words.len() - 1].len() as i32
}

#[cfg(test)]
mod tests {
    use crate::p58_length_of_last_word::length_of_last_word;

    #[test]
    fn test_one() {
        let s = String::from("Hello world");
        let expected = 5;
        let output = length_of_last_word(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = String::from("   fly me   to   the moon  ");
        let expected = 4;
        let output = length_of_last_word(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let s = String::from("luffy is still joyboy");
        let expected = 6;
        let output = length_of_last_word(s);
        assert_eq!(expected, output);
    }
}