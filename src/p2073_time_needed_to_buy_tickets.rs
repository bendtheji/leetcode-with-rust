#![allow(dead_code)]

use std::collections::VecDeque;

pub fn time_required_to_buy(tickets: Vec<i32>, mut k: i32) -> i32 {
    let mut counter = 0;
    let mut queue = VecDeque::from(tickets);

    while !queue.is_empty() {
        let no_of_people = queue.len();
        let k_curr = k;
        for i in 0..no_of_people {
            let curr = queue.pop_front().unwrap();
            let curr = curr - 1;
            if curr == 0 {
                match i as i32 {
                    // less than k
                    x if x < k_curr => {
                        k = k - 1;
                    }
                    // is k
                    x if x == k_curr => {
                        return counter + 1;
                    }
                    // more than k
                    _ => ()
                }
            } else {
                queue.push_back(curr);
            }
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::p2073_time_needed_to_buy_tickets::time_required_to_buy;

    #[test]
    fn test_one() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        let expected = 6;
        let output = time_required_to_buy(tickets, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let tickets = vec![5, 1, 1, 1];
        let k = 0;
        let expected = 8;
        let output = time_required_to_buy(tickets, k);
        assert_eq!(expected, output);
    }
}