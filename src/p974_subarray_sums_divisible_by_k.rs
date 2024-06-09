#![allow(dead_code)]

pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    // keep prefix sum currently
    // keep an array of counts of subsrrays up at index i
    // where prefix % k == remainder
    let mut prefix = 0;
    let mut result = 0;
    let mut counts = vec![0; k as usize];
    counts[0] = 1;

    for num in nums {
        prefix = ((prefix + num) % k + k) % k;
        result += counts[prefix as usize];
        counts[prefix as usize] += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::p974_subarray_sums_divisible_by_k::subarrays_div_by_k;

    #[test]
    fn test_one() {
        let nums = vec![4, 5, 0, -2, -3, 1];
        let k = 5;
        let expected = 7;
        let output = subarrays_div_by_k(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![5];
        let k = 9;
        let expected = 0;
        let output = subarrays_div_by_k(nums, k);
        assert_eq!(expected, output);
    }
}