#![allow(dead_code)]

pub fn remove_kdigits(num: String, mut k: i32) -> String {
    let mut stack = vec![];

    for c in num.chars() {
        while !stack.is_empty() && k > 0 && *stack.last().unwrap() > c {
            stack.pop();
            k -= 1;
        }
        stack.push(c);
    }

    while k > 0 {
        stack.pop();
        k -= 1;
    }

    let result = stack.into_iter().collect::<String>();
    let result = result.trim_start_matches('0');
    match result {
        "" => "0".to_string(),
        _ => result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::p402_remove_k_digits::remove_kdigits;

    #[test]
    fn test_one() {
        let num = String::from("1432219");
        let k = 3;
        let expected = String::from("1219");
        let output = remove_kdigits(num, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let num = String::from("10200");
        let k = 1;
        let expected = String::from("200");
        let output = remove_kdigits(num, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let num = String::from("10");
        let k = 2;
        let expected = String::from("0");
        let output = remove_kdigits(num, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_four() {
        let num = String::from("10");
        let k = 1;
        let expected = String::from("0");
        let output = remove_kdigits(num, k);
        assert_eq!(expected, output);
    }
}