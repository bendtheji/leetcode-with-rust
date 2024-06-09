#![allow(dead_code)]

pub fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();
    let half = length / 2;
    for i in 0..half {
        let temp = s[i];
        s[i] = s[s.len() - 1 - i];
        s[length - 1 - i] = temp;
    }
}

#[cfg(test)]
mod tests {
    use crate::p344_reverse_string::reverse_string;

    #[test]
    fn test_one() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let expected = vec!['o', 'l', 'l', 'e', 'h'];
        reverse_string(&mut s);
        assert_eq!(expected, s);
    }

    #[test]
    fn test_two() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h']; 
        let expected = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        reverse_string(&mut s);
        assert_eq!(expected, s);
    }
}