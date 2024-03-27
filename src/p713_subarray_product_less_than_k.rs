#[allow(dead_code)]
pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k <= 1 { return 0; }

    let n = nums.len();

    let mut total_count = 0;
    let mut curr = 1;

    let mut left = 0;
    let mut right = 0;

    while right < n {
        curr *= nums[right];

        while curr >= k {
            curr /= nums[left];
            left += 1;
        }

        total_count += right - left + 1;
        right += 1;
    }
    total_count as i32
}

#[cfg(test)]
mod tests {
    use crate::p713_subarray_product_less_than_k::num_subarray_product_less_than_k;

    #[test]
    fn test_one() {
        let nums = vec![10, 5, 2, 6];
        let k = 100;
        let expected = 8;
        let output = num_subarray_product_less_than_k(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 2, 3];
        let k = 0;
        let expected = 0;
        let output = num_subarray_product_less_than_k(nums, k);
        assert_eq!(expected, output);
    }
}