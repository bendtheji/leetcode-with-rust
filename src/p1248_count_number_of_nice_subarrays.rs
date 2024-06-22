#![allow(dead_code)]

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut indices = vec![];

    for i in 0..nums.len() {
        if nums[i] % 2 == 1 { indices.push(i); }
    }

    let mut left = 0;
    let mut right = (k - 1) as usize;
    let mut result = 0;

    while right < indices.len() {
        let cur_start = indices[left] as i32;
        let mut prev_index = -1;
        if left != 0 { prev_index = indices[left - 1] as i32; }

        let cur_end = indices[right] as i32;
        let mut next_index = nums.len() as i32;
        if right != indices.len() - 1 { next_index = indices[right + 1] as i32; }

        result += (cur_start - prev_index) * (next_index - cur_end);

        left += 1;
        right += 1;
    }
    result
}

// get all the indices of the odd nums
// result += (cur_start - prev_index) * (next_index - cur_end)
// if cur_start idx is 0 in list of indices, then prev_index is -1
// if cur_end idx is indices.len()-1, then next_index is nums.len()

#[cfg(test)]
mod tests {
    use crate::p1248_count_number_of_nice_subarrays::number_of_subarrays;

    #[test]
    fn test_one() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 3;
        let expected = 2;
        let output = number_of_subarrays(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![2, 4, 6];
        let k = 1;
        let expected = 0;
        let output = number_of_subarrays(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
        let k = 2;
        let expected = 16;
        let output = number_of_subarrays(nums, k);
        assert_eq!(expected, output);
    }
}