#![allow(dead_code)]

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    xor_sum(&nums, 0, 0)
}

fn xor_sum(nums: &Vec<i32>, index: usize, current_xor: i32) -> i32 {
    if index == nums.len() {
        return current_xor;
    }

    let with_curr = xor_sum(nums, index + 1, current_xor ^ nums[index]);

    let without_curr = xor_sum(nums, index + 1, current_xor);

    with_curr + without_curr
}

#[cfg(test)]
mod tests {
    use crate::p1863_sum_of_all_subset_xor_totals::subset_xor_sum;

    #[test]
    fn test_one() {
        let nums = vec![1, 3];
        let expected = 6;
        let output = subset_xor_sum(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![5, 1, 6];
        let expected = 28;
        let output = subset_xor_sum(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![3, 4, 5, 6, 7, 8];
        let expected = 480;
        let output = subset_xor_sum(nums);
        assert_eq!(expected, output);
    }
}