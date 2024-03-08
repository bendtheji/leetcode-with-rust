#![allow(dead_code)]

pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut start = new_interval[0];
    let mut end = new_interval[1];

    let mut curr = 0;
    while curr < intervals.len() && intervals[curr][1] < start {
        result.push(intervals[curr].clone());
        curr = curr + 1;
    }

    while curr < intervals.len() && intervals[curr][0] <= end {
        start = start.min(intervals[curr][0]);
        end = end.max(intervals[curr][1]);
        curr = curr + 1;
    }

    result.push(vec![start, end]);
    for i in curr..intervals.len() { result.push(intervals[i].clone()); }
    result
}

#[cfg(test)]
mod tests {
    use crate::p57_insert_interval::insert;

    #[test]
    fn test_one() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]];
        let new_interval = vec![4, 8];
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        let output = insert(intervals, new_interval);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let expected = vec![vec![1, 5], vec![6, 9]];
        let output = insert(intervals, new_interval);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let intervals = vec![];
        let new_interval = vec![5, 7];
        let expected = vec![vec![5, 7]];
        let output = insert(intervals, new_interval);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_four() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 7];
        let expected = vec![vec![1, 7]];
        let output = insert(intervals, new_interval);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_five() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![5, 7];
        let expected = vec![vec![1, 7]];
        let output = insert(intervals, new_interval);
        assert_eq!(expected, output);
    }
}