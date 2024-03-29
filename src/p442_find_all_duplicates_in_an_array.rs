#![allow(dead_code)]

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..nums.len() {
        let num = nums[i].abs();
        let index = (num - 1) as usize;
        if nums[index] < 0 { result.push(num); } else { nums[index] = -nums[index]; }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::p442_find_all_duplicates_in_an_array::find_duplicates;

    #[test]
    fn test_one() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![2, 3];
        let output = find_duplicates(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![1, 1, 2];
        let expected = vec![1];
        let output = find_duplicates(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![1];
        let expected = Vec::<i32>::new();
        let output = find_duplicates(nums);
        assert_eq!(expected, output);
    }
}