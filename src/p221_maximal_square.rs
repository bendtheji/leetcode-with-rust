#![allow(dead_code)]

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max = 0;
    let mut matrix: Vec<Vec<u32>> = matrix
        .into_iter()
        .map(|x| x.into_iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in 0..n {
            if i == 0 || j == 0 {
                max = std::cmp::max(max, matrix[i][j]);
            } else {
                if matrix[i][j] == 1 {
                    let one = matrix[i - 1][j - 1];
                    let two = matrix[i][j - 1];
                    let three = matrix[i - 1][j];

                    let min_square_length = one.min(two).min(three) + 1;
                    max = std::cmp::max(max, min_square_length * min_square_length);
                    matrix[i][j] = min_square_length;
                }
            }
        }
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use crate::p221_maximal_square::maximal_square;

    #[test]
    fn test_one() {
        let matrix = vec![vec!['1', '0', '1', '0', '0'], vec!['1', '0', '1', '1', '1'], vec!['1', '1', '1', '1', '1'], vec!['1', '0', '0', '1', '0']];
        let expected = 4;
        let output = maximal_square(matrix);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let matrix = vec![vec!['0', '1'], vec!['1', '0']];
        let expected = 1;
        let output = maximal_square(matrix);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let matrix = vec![vec!['0']];
        let expected = 0;
        let output = maximal_square(matrix);
        assert_eq!(expected, output);
    }
}