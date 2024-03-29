#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {

    // sort by the start timing first
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

    // create a min heap to store the end timings for the meetings
    // in rust binaryheap is max heap so we need to reverse our inputs
    let mut min_heap = BinaryHeap::<Reverse<i32>>::new();

    for interval in intervals {
        let start_timing = interval[0];
        // either start timing for current interval is more than the min heap
        // which means we can use the same meeting room
        match min_heap.peek() {
            Some(x) if start_timing >= x.0 => {
                min_heap.pop();
                min_heap.push(Reverse(interval[1]));
            }
            // or less than the min
            // which means we need another room
            _ => {
                min_heap.push(Reverse(interval[1]));
            }
        }
    }

    min_heap.len() as i32
}

#[cfg(test)]
mod tests {
    use crate::p253_meeting_rooms_ii::min_meeting_rooms;

    #[test]
    fn test_one() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        let expected = 2;
        let output = min_meeting_rooms(intervals);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        let expected = 1;
        let output = min_meeting_rooms(intervals);
        assert_eq!(expected, output);
    }
}