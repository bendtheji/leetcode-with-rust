#![allow(dead_code)]

pub fn beautiful_subsets(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();

    let mut counts = vec![0; (nums[nums.len() - 1] + 1) as usize];

    backtracking(&nums, 0, &mut counts, k) - 1
}

fn backtracking(nums: &Vec<i32>, index: usize, counts: &mut Vec<i32>, k: i32) -> i32 {
    if index == nums.len() {
        return 1;
    }

    let not_taken = backtracking(nums, index + 1, counts, k);
    let mut taken = 0;

    let curr_num = nums[index] as usize;

    if curr_num <= k as usize || counts[curr_num - (k as usize)] == 0 {
        counts[curr_num] += 1;
        taken = backtracking(nums, index + 1, counts, k);
        counts[curr_num] -= 1;
    }
    not_taken + taken
}

#[cfg(test)]
mod tests {
    use crate::p2597_the_number_of_beautiful_subsets::beautiful_subsets;

    #[test]
    fn test_one() {
        let nums = vec![2, 4, 6];
        let k = 2;
        let expected = 4;
        let output = beautiful_subsets(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1];
        let k = 1;
        let expected = 1;
        let output = beautiful_subsets(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![10, 4, 5, 7, 2, 1];
        let k = 3;
        let expected = 23;
        let output = beautiful_subsets(nums, k);
        assert_eq!(expected, output);
    }
}