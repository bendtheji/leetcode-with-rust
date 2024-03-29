#![allow(dead_code)]

pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        if nums[i] <= 0 || nums[i] > n as i32 {
            nums[i] = (n + 1) as i32;
        }
    }

    for i in 0..n {
        let mut index = nums[i].abs() as usize;
        if index > n {
            continue;
        }
        index = index - 1;
        if nums[index] > 0 {
            nums[index] = -nums[index]
        }
    }

    for i in 0..n {
        if nums[i] > 0 {
            return (i + 1) as i32;
        }
    }

    (n + 1) as i32
}

#[cfg(test)]
mod tests {
    use crate::p41_first_missing_positive::first_missing_positive;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 0];
        let expected = 3;
        let output = first_missing_positive(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![3, 4, -1, 1];
        let expected = 2;
        let output = first_missing_positive(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![7, 8, 9, 11, 12];
        let expected = 1;
        let output = first_missing_positive(nums);
        assert_eq!(expected, output);
    }
}