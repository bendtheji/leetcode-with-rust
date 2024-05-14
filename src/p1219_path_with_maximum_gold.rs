#![allow(dead_code)]

pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            backtrack(&mut grid, 0, &mut max, i as i32, j as i32);
        }
    }

    max
}

fn backtrack(grid: &mut Vec<Vec<i32>>, mut curr_gold: i32, max: &mut i32, x: i32, y: i32) {
    if grid[x as usize][y as usize] == 0 {
        return;
    }

    let curr_cell = grid[x as usize][y as usize];
    curr_gold = curr_gold + curr_cell;

    if curr_gold > *max {
        *max = curr_gold;
    }

    grid[x as usize][y as usize] = 0;

    let directions = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    for direction in directions {
        let x = x + direction.0;
        let y = y + direction.1;

        if x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32 {
            backtrack(grid, curr_gold, max, x, y);
        }
    }

    grid[x as usize][y as usize] = curr_cell;
}
// backtracking

// at each cell, we add the current cell value
// four directions to check from the cell
// if zero then we can go back
// once we travel

#[cfg(test)]
mod tests {
    use crate::p1219_path_with_maximum_gold::get_maximum_gold;

    #[test]
    fn test_one() {
        let grid = vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]];
        let expected = 24;
        let output = get_maximum_gold(grid);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let grid = vec![vec![1, 0, 7], vec![2, 0, 6], vec![3, 4, 5], vec![0, 3, 0], vec![9, 0, 20]];
        let expected = 28;
        let output = get_maximum_gold(grid);
        assert_eq!(expected, output);
    }
}