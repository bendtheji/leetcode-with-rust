#[allow(dead_code)]
pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_unstable();
    let mut result = 0;
    let mut minus = 0;
    for _ in 0..k {
        let curr = happiness.pop().unwrap() as i64;
        let curr = std::cmp::max(curr - minus, 0);
        result += curr;
        minus += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::p3075_maximize_happiness_of_selected_children::maximum_happiness_sum;

    #[test]
    fn test_one() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let expected = 4;
        let output = maximum_happiness_sum(happiness, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let happiness = vec![2, 3, 4, 5];
        let k = 1;
        let expected = 5;
        let output = maximum_happiness_sum(happiness, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let happiness = vec![1, 1, 1, 1];
        let k = 2;
        let expected = 1;
        let output = maximum_happiness_sum(happiness, k);
        assert_eq!(expected, output);
    }
}