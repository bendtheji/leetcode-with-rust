#![allow(dead_code)]

pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut map = vec![0; 3];
    let length = nums.len();

    for num in nums.iter() {
        map[*num as usize] += 1;
    }

    let mut ptr = 0;
    let mut index = 0;
    while ptr < length {
        if map[index] > 0 {
            nums[ptr] = index as i32;
            map[index] -= 1;
            ptr += 1;
        } else {
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p75_sort_colors::sort_colors;

    #[test]
    fn test_one() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test_two() {
        let mut nums = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }
}