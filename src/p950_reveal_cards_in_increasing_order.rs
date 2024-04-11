#![allow(dead_code)]

use std::collections::VecDeque;

pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
    deck.sort_unstable();

    let mut queue = VecDeque::new();

    while !deck.is_empty() {
        // take tail and push to the front
        if let Some(tail) = queue.pop_back() {
            queue.push_front(tail);
        }

        // pop from deck and add it to front
        if let Some(max) = deck.pop() {
            queue.push_front(max);
        }
    }
    Vec::from(queue)
}

#[cfg(test)]
mod tests {
    use crate::p950_reveal_cards_in_increasing_order::deck_revealed_increasing;

    #[test]
    fn test_one() {
        let deck = vec![17, 13, 11, 2, 3, 5, 7];
        let expected = vec![2, 13, 3, 11, 5, 17, 7];
        let output = deck_revealed_increasing(deck);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let deck = vec![1, 1000];
        let expected = vec![1, 1000];
        let output = deck_revealed_increasing(deck);
        assert_eq!(expected, output);
    }
}