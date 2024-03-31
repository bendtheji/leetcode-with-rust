#![allow(dead_code)]

use std::cmp;
use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut frequencies = HashMap::new();

    let mut max_length = 0;

    for right in 0..nums.len() {
        let cur_elem = nums[right];

        *frequencies.entry(cur_elem).or_insert(0) += 1;

        while let Some(&count) = frequencies.get(&cur_elem) {
            if count > k {
                frequencies.entry(nums[left]).and_modify(|e| { *e -= 1 });
                left += 1;
            } else {
                break;
            }
        }

        max_length = cmp::max(max_length, right - left + 1);
    }

    max_length as i32
}

// for each element (right pointer) we add it into the hashmap
// while count of element is > k
// we should remove the left pointer element from the hashmap
// if count of the current element (right pointer) is equal to k
// we stop moving the left pointer
// max_length = cmp::max(max_length, right - left + 1)

#[cfg(test)]
mod tests {
    use crate::p2958_length_of_longest_subarray_with_at_most_k_frequency::max_subarray_length;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;
        let expected = 6;
        let output = max_subarray_length(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];
        let k = 1;
        let expected = 2;
        let output = max_subarray_length(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![5, 5, 5, 5, 5, 5, 5];
        let k = 4;
        let expected = 4;
        let output = max_subarray_length(nums, k);
        assert_eq!(expected, output);
    }
}