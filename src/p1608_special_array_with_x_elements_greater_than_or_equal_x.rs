#![allow(dead_code)]

pub fn special_array(nums: Vec<i32>) -> i32 {
    let mut count = vec![0; nums.len() + 1];

    for num in &nums {
        let num = *num as usize;
        count[std::cmp::min(num, nums.len())] += 1;
    }

    let mut no_of_elems = 0;
    for i in (0..=nums.len()).rev() {
        no_of_elems += count[i];
        if i == no_of_elems {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::p1608_special_array_with_x_elements_greater_than_or_equal_x::special_array;

    #[test]
    fn test_one() {
        let nums = vec![3, 5];
        let expected = 2;
        let output = special_array(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![0, 0];
        let expected = -1;
        let output = special_array(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let nums = vec![0, 4, 3, 0, 4];
        let expected = 3;
        let output = special_array(nums);
        assert_eq!(expected, output);
    }
}