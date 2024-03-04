#![allow(dead_code)]

use std::cmp;

pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
    let mut tokens = tokens;
    let mut power = power;

    tokens.sort_unstable();

    if tokens.is_empty() { return 0; }

    let mut left = 0;
    let mut right = tokens.len() - 1;

    let mut cur_score = 0;
    let mut max_score = 0;

    while left <= right {
        if tokens[left] <= power {
            power -= tokens[left];
            cur_score += 1;
            left += 1;
            max_score = cmp::max(cur_score, max_score);
        } else if cur_score >= 1 {
            cur_score -= 1;
            power += tokens[right];
            right -= 1;
        } else {
            return max_score;
        }
    }
    max_score
}

#[cfg(test)]
mod tests {
    use crate::p948_bag_of_tokens::bag_of_tokens_score;

    #[test]
    fn test_one() {
        let expected = 0;
        let output = bag_of_tokens_score(vec![100], 50);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let expected = 1;
        let output = bag_of_tokens_score(vec![200, 100], 150);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let expected = 2;
        let output = bag_of_tokens_score(vec![100, 200, 300, 400], 200);
        assert_eq!(expected, output);
    }
}