#![allow(dead_code)]

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![];
    let mut matrix = matrix;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let m = matrix.len();
    let n = matrix[0].len();

    let total_cells = m * n;
    let mut cur_row = 0 as i32;
    let mut cur_col = 0 as i32;

    let mut cur_direction = 0;

    result.push(matrix[cur_row as usize][cur_col as usize]);
    matrix[cur_row as usize][cur_col as usize] = -101;

    for _ in 1..total_cells {
        while !valid_direction(&matrix, directions[cur_direction], cur_row, cur_col, m as i32, n as i32) {
            cur_direction = (cur_direction + 1) % 4;
        }

        cur_row = cur_row + directions[cur_direction].0;
        cur_col = cur_col + directions[cur_direction].1;

        result.push(matrix[cur_row as usize][cur_col as usize]);

        matrix[cur_row as usize][cur_col as usize] = -101;
    }
    result
}

pub fn valid_direction(matrix: &Vec<Vec<i32>>, direction: (i32, i32), cur_row: i32, cur_col: i32, m: i32, n: i32) -> bool {
    let next_row = cur_row + direction.0;
    let next_col = cur_col + direction.1;

    next_row >= 0 && next_row < m && next_col >= 0 && next_col < n && matrix[next_row as usize][next_col as usize] != -101
}

#[cfg(test)]
mod tests {
    use crate::p54_spiral_matrix::spiral_order;

    #[test]
    fn test_one() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        let output = spiral_order(matrix);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        let output = spiral_order(matrix);
        assert_eq!(expected, output);
    }
}