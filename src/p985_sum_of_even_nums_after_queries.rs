#![allow(dead_code)]

pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // get sum of even numbers first
    let mut result = vec![];
    let mut nums = nums;
    let mut sum = nums.iter().filter(|num| *num % 2 == 0).sum();

    // each num can change in 4 ways
    for query in queries {
        let val = query[0];
        let index = query[1] as usize;
        let old_val = nums[index];
        let new_val = nums[index] + val;

        let is_old_even = old_val % 2 == 0;
        let is_new_even = new_val % 2 == 0;
        match (is_old_even, is_new_even) {
            (true, true) => {
                sum = sum + (new_val - old_val);
            },
            (true, false) => {
                sum -= old_val;
            },
            (false, true) => {
                sum += new_val;
            },
            _ => ()
        }
        nums[index] = new_val;
        result.push(sum);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::p985_sum_of_even_nums_after_queries::sum_even_after_queries;

    #[test]
    fn test_one() {
        let expected = vec![8,6,2,4];
        let output = sum_even_after_queries(vec![1,2,3,4], vec![vec![1,0],vec![-3,1],vec![-4,0],vec![2,3]]);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = vec![0];
        let output = sum_even_after_queries(vec![1], vec![vec![4,0]]);
        assert_eq!(expected, output);
    }
}