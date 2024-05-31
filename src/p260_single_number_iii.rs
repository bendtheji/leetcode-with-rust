#![allow(dead_code)]

pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let final_xor = nums.iter().fold(0, |mut a, x| {
        a ^= x;
        a
    });
    let diff = final_xor & (-final_xor);
    let x = nums.iter().filter(|&x| x & diff != 0).fold(0, |mut a, x| {
        a ^= x;
        a
    });
    vec![x, final_xor ^ x]
}

#[cfg(test)]
mod tests {
    use crate::p260_single_number_iii::single_number;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 1, 3, 2, 5];
        let expected = vec![3, 5];
        let output = single_number(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![-1, 0];
        let expected = vec![-1, 0];
        let output = single_number(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![0, 1];
        let expected = vec![1, 0];
        let output = single_number(nums);
        assert_eq!(expected, output);
    }
}