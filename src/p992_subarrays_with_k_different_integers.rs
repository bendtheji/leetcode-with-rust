#![allow(dead_code)]

use std::collections::HashMap;

pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
    sliding_window_at_most(&nums, k) - sliding_window_at_most(&nums, k - 1)
}

pub fn sliding_window_at_most(nums: &Vec<i32>, k: i32) -> i32 {
    let mut frequencies = HashMap::new();
    let mut left = 0;
    let mut total_count = 0;

    for right in 0..nums.len() {
        *frequencies.entry(nums[right]).or_insert(0) += 1;

        while frequencies.len() as i32 > k {
            frequencies.entry(nums[left]).and_modify(|e| { *e -= 1 });
            match frequencies.get(&nums[left]) {
                Some(&count) if count == 0 => { frequencies.remove(&nums[left]); }
                _ => ()
            }
            left += 1;
        }

        total_count += right - left + 1;
    }
    total_count as i32
}

#[cfg(test)]
mod tests {
    use crate::p992_subarrays_with_k_different_integers::subarrays_with_k_distinct;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 1, 2, 3];
        let k = 2;
        let expected = 7;
        let output = subarrays_with_k_distinct(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 2, 1, 3, 4];
        let k = 3;
        let expected = 3;
        let output = subarrays_with_k_distinct(nums, k);
        assert_eq!(expected, output);
    }
}