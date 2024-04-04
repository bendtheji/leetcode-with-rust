use std::cmp;

pub fn max_depth(s: String) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for c in s.chars() {
        match c {
            '(' => {
                count += 1;
                max = cmp::max(max, count);
            }
            ')' => { count -= 1; }
            _ => ()
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::p1614_maximum_nesting_depth_of_the_parentheses::max_depth;

    #[test]
    fn test_one() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        let expected = 3;
        let output = max_depth(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = "(1)+((2))+(((3)))".to_string();
        let expected = 3;
        let output = max_depth(s);
        assert_eq!(expected, output);
    }
}