#![allow(dead_code)]

pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let mut start = start;
    let mut goal = goal;
    let mut min = 0;

    while start > 0 || goal > 0 {
        match (start & 1, goal & 1) {
            (1, 0) | (0, 1) => { min += 1; }
            _ => (),
        }
        start >>= 1;
        goal >>= 1;
    }

    min
}

#[cfg(test)]
mod tests {
    use crate::p2220_minimum_bit_flips_to_convert_number::min_bit_flips;

    #[test]
    fn test_one() {
        let start = 10;
        let goal = 7;
        let expected = 3;
        let output = min_bit_flips(start, goal);
        assert_eq!(expected, output)
    }

    #[test]
    fn test_two() {
        let start = 3;
        let goal = 4;
        let expected = 3;
        let output = min_bit_flips(start, goal);
        assert_eq!(expected, output)
    }
}