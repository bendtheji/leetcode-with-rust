#![allow(dead_code)]

pub fn make_good(s: String) -> String {
    let bytes = s.chars().map(|c| c as i8).collect::<Vec<i8>>();
    let mut stack = vec![];

    for i in 0..bytes.len() {
        let cur_char = bytes[i];

        if let Some(last_char) = stack.last() {
            if (last_char - cur_char).abs() == 32 { stack.pop(); } else { stack.push(cur_char); }
        } else { stack.push(cur_char); }
    }

    stack.into_iter().map(|x| x as u8).map(|x| x as char).collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::p1544_make_the_string_great::make_good;

    #[test]
    fn test_one() {
        let s = "leEeetcode".to_string();
        let expected = "leetcode".to_string();
        let output = make_good(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = "abBAcC".to_string();
        let expected = "".to_string();
        let output = make_good(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let s = "s".to_string();
        let expected = "s".to_string();
        let output = make_good(s);
        assert_eq!(expected, output);
    }
}