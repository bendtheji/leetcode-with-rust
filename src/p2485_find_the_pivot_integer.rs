#![allow(dead_code)]

pub fn pivot_integer(n: i32) -> i32 {
    let mut left: i32 = (1..=n).sum();
    let mut right = n;
    if left == right { return n; }

    for i in (1..=n - 1).rev() {
        left -= i + 1;
        right += i;
        if left == right { return i; }
    }

    -1
}

#[cfg(test)]
mod tests {
    use crate::p2485_find_the_pivot_integer::pivot_integer;

    #[test]
    fn test_one() {
        let n = 8;
        let expected = 6;
        let output = pivot_integer(n);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let n = 1;
        let expected = 1;
        let output = pivot_integer(n);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let n = 4;
        let expected = -1;
        let output = pivot_integer(n);
        assert_eq!(expected, output);
    }
}