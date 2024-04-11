#![allow(dead_code)]

use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut map = HashMap::new();

    for num in nums2 {
        while stack.last().is_some() && *stack.last().unwrap() < num {
            let top = stack.pop().unwrap();
            map.insert(top, num);
        }
        stack.push(num);
    }

    let mut ans = vec![];
    for num in nums1 {
        match map.get(&num) {
            Some(&x) => ans.push(x),
            None => ans.push(-1)
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::p496_next_greater_element_i::next_greater_element;

    #[test]
    fn test_one() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let expected = vec![-1, 3, -1];
        let output = next_greater_element(nums1, nums2);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let expected = vec![3, -1];
        let output = next_greater_element(nums1, nums2);
        assert_eq!(expected, output);
    }
}