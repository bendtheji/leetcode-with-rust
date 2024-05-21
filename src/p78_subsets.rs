#![allow(dead_code)]

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut subsets = vec![];
    let mut curr = vec![];

    create_subset(0, &mut curr, &mut subsets, &nums);

    subsets
}

fn create_subset(index: usize, curr: &mut Vec<i32>, subsets: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
    if index == nums.len() {
        subsets.push(curr.clone());
        return;
    }

    curr.push(nums[index]);
    create_subset(index + 1, curr, subsets, nums);
    curr.pop();

    create_subset(index + 1, curr, subsets, nums);
}

#[cfg(test)]
mod tests {
    use crate::p78_subsets::subsets;

    #[test]
    fn test_one() {
        let nums = vec![1, 2, 3];
        let expected = vec![vec![1, 2, 3], vec![1, 2], vec![1, 3], vec![1], vec![2, 3], vec![2], vec![3], vec![]];
        let output = subsets(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let nums = vec![0];
        let expected = vec![vec![0], vec![]];
        let output = subsets(nums);
        assert_eq!(expected, output);
    }
}