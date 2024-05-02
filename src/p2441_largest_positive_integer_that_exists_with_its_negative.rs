#![allow(dead_code)]

use std::collections::HashSet;

pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();

    let mut ans = -1;

    for num in nums {
        let abs_num = num.abs();

        if abs_num > ans && seen.contains(&-num) {
            ans = abs_num;
        }

        seen.insert(num);
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::p2441_largest_positive_integer_that_exists_with_its_negative::find_max_k;

    #[test]
    fn test_one() {
        let nums = vec![-1, 2, -3, 3];
        let expected = 3;
        let output = find_max_k(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![-1, 10, 6, 7, -7, 1];
        let expected = 7;
        let output = find_max_k(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![-10, 8, 6, 7, -2, -3];
        let expected = -1;
        let output = find_max_k(nums);
        assert_eq!(expected, output);
    }
}