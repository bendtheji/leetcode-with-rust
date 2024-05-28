#![allow(dead_code)]

pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let diffs = s.chars().zip(t.chars()).map(|(s_char, t_char)| (t_char as i32 - s_char as i32).abs()).collect::<Vec<i32>>();

    let mut max_length = 0;
    let mut curr_cost = 0;
    let mut left = 0;
    let mut right = 0;

    while left < s.len() {
        while right < s.len() && curr_cost <= max_cost {
            curr_cost += diffs[right];
            right += 1;
            if curr_cost <= max_cost { max_length = std::cmp::max(max_length, right - left); }
        }

        while curr_cost > max_cost || right == s.len() && left < s.len() {
            curr_cost -= diffs[left];
            left += 1;
            if curr_cost <= max_cost { max_length = std::cmp::max(max_length, right - left); }
        }
    }
    max_length as i32
}

#[cfg(test)]
mod tests {
    use crate::p1208_get_equal_substrings_within_budget::equal_substring;

    #[test]
    fn test_one() {
        let s = String::from("abcd");
        let t = String::from("bcdf");
        let max_cost = 3;
        let expected = 3;
        let output = equal_substring(s, t, max_cost);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let s = String::from("abcd");
        let t = String::from("cdef");
        let max_cost = 3;
        let expected = 1;
        let output = equal_substring(s, t, max_cost);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let s = String::from("abcd");
        let t = String::from("acde");
        let max_cost = 0;
        let expected = 1;
        let output = equal_substring(s, t, max_cost);
        assert_eq!(expected, output);
    }
}