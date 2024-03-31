#![allow(dead_code)]

use std::cmp;

pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut start = 0;

    let mut ans = 0;

    let mut cur_min_idx = None;
    let mut cur_max_idx = None;

    for ptr in 0..nums.len() {
        let cur_elem = nums[ptr];

        if cur_elem < min_k || cur_elem > max_k {
            cur_min_idx = None;
            cur_max_idx = None;
            start = ptr + 1;
            continue;
        }

        if cur_elem == min_k { cur_min_idx = Some(ptr); }
        if cur_elem == max_k { cur_max_idx = Some(ptr); }

        if let (Some(min_idx), Some(max_idx)) = (cur_min_idx, cur_max_idx) {
            let lower_idx = cmp::min(min_idx, max_idx);
            ans += lower_idx - start + 1;
        }
    }
    ans as i64
}

#[cfg(test)]
mod tests {
    use crate::p2444_count_subarrays_with_fixed_bounds::count_subarrays;

    #[test]
    fn test_one() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        let expected = 2;
        let output = count_subarrays(nums, min_k, max_k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        let expected = 10;
        let output = count_subarrays(nums, min_k, max_k);
        assert_eq!(expected, output);
    }
}