#![allow(dead_code)]

use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut width = (height.len() - 1) as i32;
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max = cmp::min(height[left], height[right]) * width;

    while left < right {
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
        width -= 1;
        max = cmp::max(max, cmp::min(height[left], height[right]) * width);
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::p11_container_with_most_water::max_area;

    #[test]
    fn test_one() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }

    #[test]
    fn test_two() {
        assert_eq!(max_area(vec![1,1]), 1);
    }
}