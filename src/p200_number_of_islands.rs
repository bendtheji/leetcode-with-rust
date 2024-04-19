#![allow(dead_code)]

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut no_of_islands = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                no_of_islands += 1;
                dfs(&mut grid, i, j);
            }
        }
    }
    no_of_islands
}

fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    if grid[row][col] == '1' {
        grid[row][col] = '2';
        if row > 0 { dfs(grid, row - 1, col); }
        if row < grid.len() - 1 { dfs(grid, row + 1, col); }
        if col > 0 { dfs(grid, row, col - 1); }
        if col < grid[0].len() - 1 { dfs(grid, row, col + 1); }
    }
}

#[cfg(test)]
mod tests {
    use crate::p200_number_of_islands::num_islands;

    #[test]
    fn test_one() {
        let input = vec![vec!['1', '1', '1', '1', '0'], vec!['1', '1', '0', '1', '0'], vec!['1', '1', '0', '0', '0'], vec!['0', '0', '0', '0', '0']];
        let expected = 1;
        let output = num_islands(input);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let input = vec![vec!['1', '1', '0', '0', '0'], vec!['1', '1', '0', '0', '0'], vec!['0', '0', '1', '0', '0'], vec!['0', '0', '0', '1', '1']];
        let expected = 3;
        let output = num_islands(input);
        assert_eq!(expected, output);
    }
}