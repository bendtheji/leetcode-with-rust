#![allow(dead_code)]

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let max_element = *(nums.iter().max().unwrap());

    let mut ans = 0;
    let mut start = 0;
    let mut max_elements_in_window = 0;

    for end in 0..nums.len() {
        if nums[end] == max_element {
            max_elements_in_window += 1;
        }
        while k == max_elements_in_window {
            if nums[start] == max_element {
                max_elements_in_window -= 1;
            }
            start += 1;
        }
        ans += start;
    }

    ans as i64
}

#[cfg(test)]
mod tests {
    use crate::p2962_count_subarrays_where_max_element_appears_at_least_k_times::count_subarrays;

    #[test]
    fn test_one() {
        let nums = vec![1, 3, 2, 3, 3];
        let k = 2;
        let expected = 6;
        let output = count_subarrays(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 4, 2, 1];
        let k = 3;
        let expected = 0;
        let output = count_subarrays(nums, k);
        assert_eq!(expected, output);
    }
}