#![allow(dead_code)]

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut start = nums.into_iter().fold(0, |a, i| {
        a ^ i
    });
    let mut goal = k;
    let mut min = 0;

    while start > 0 || goal > 0 {
        match (start & 1, goal & 1) {
            (1, 0) | (0, 1) => { min += 1; }
            _ => (),
        }
        start >>= 1;
        goal >>= 1;
    }

    min
}

#[cfg(test)]
mod tests {
    use crate::p2997_minimum_number_of_operations_to_make_array_xor_equal_to_k::min_operations;

    #[test]
    fn test_one() {
        let nums = vec![2, 1, 3, 4];
        let k = 1;
        let expected = 2;
        let output = min_operations(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![2, 0, 2, 0];
        let k = 0;
        let expected = 0;
        let output = min_operations(nums, k);
        assert_eq!(expected, output);
    }
}