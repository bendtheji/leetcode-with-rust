#![allow(dead_code)]

pub fn maximum_odd_binary_number(s: String) -> String {
    let mut ans = String::with_capacity(s.len());
    let mut count = s.chars().fold(0, |acc, cur_char| match cur_char {
        '1' => acc + 1,
        _ => acc
    });
    for _ in 0..s.len() - 1 {
        if count > 1 {
            ans.push('1');
            count -= 1;
        } else { ans.push('0') }
    }
    ans.push('1');
    ans
}

#[cfg(test)]
mod tests {
    use crate::p2864_maximum_odd_binary::maximum_odd_binary_number;

    #[test]
    fn test_one() {
        let expected = "001".to_string();
        let output = maximum_odd_binary_number("010".to_string());
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = "1001".to_string();
        let output = maximum_odd_binary_number("0101".to_string());
        assert_eq!(expected, output);
    }
}