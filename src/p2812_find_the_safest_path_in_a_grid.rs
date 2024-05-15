#![allow(dead_code)]

use std::collections::VecDeque;

pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
    // find min distances
    find_min_distances(&mut grid);

    get_safeness_factor(&grid)
}


fn find_min_distances(grid: &mut Vec<Vec<i32>>) {
    let m = grid.len();
    let n = grid[0].len();

    let mut queue = VecDeque::new();

    // find the ones in the grid, and mark them as zero
    // put them into a queue
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                grid[i][j] = 0;
                queue.push_back((i as i32, j as i32));
            } else {
                // anything else should be marked as -1 to indicate unvisited
                grid[i][j] = -1;
            }
        }
    }

    while !queue.is_empty() {
        let cell = queue.pop_front().unwrap();
        let x = cell.0;
        let y = cell.1;
        let cell_val = grid[x as usize][y as usize];

        let directions = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for direction in directions {
            if direction.0 >= 0 && direction.0 < m as i32 && direction.1 >= 0 && direction.1 < n as i32 && grid[direction.0 as usize][direction.1 as usize] == -1 {
                grid[direction.0 as usize][direction.1 as usize] = cell_val + 1;
                queue.push_back(direction);
            }
        }
    }
}

fn get_safeness_factor(grid: &Vec<Vec<i32>>) -> i32 {
    let mut left = 0;
    let mut right = grid.len() as i32;
    let mut res = -1;

    while left <= right {
        let mid = (right - left) / 2 + left;
        let can_travel = can_travel(grid, mid);
        if can_travel {
            res = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    res
}

fn can_travel(grid: &Vec<Vec<i32>>, threshold: i32) -> bool {
    let m = grid.len();
    let n = grid[0].len();

    if grid[0][0] < threshold || grid[m - 1][n - 1] < threshold {
        return false;
    }

    let starting_point = (0, 0);
    let mut queue = VecDeque::<(i32, i32)>::new();
    let mut visited = vec![vec![false; n]; m];

    let m = m as i32;
    let n = n as i32;
    queue.push_back(starting_point);
    visited[0][0] = true;

    while !queue.is_empty() {
        let cell = queue.pop_front().unwrap();
        let x = cell.0;
        let y = cell.1;
        if cell.0 == m - 1 && cell.1 == n - 1 { return true; }

        let directions = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for direction in directions {
            let x = direction.0;
            let y = direction.1;
            if x >= 0 && x < m && y >= 0 && y < n && grid[x as usize][y as usize] >= threshold && !visited[x as usize][y as usize] {
                visited[x as usize][y as usize] = true;
                queue.push_back(direction);
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::p2812_find_the_safest_path_in_a_grid::maximum_safeness_factor;

    #[test]
    fn test_one() {
        let grid = vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        let expected = 0;
        let output = maximum_safeness_factor(grid);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_two() {
        let grid = vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]];
        let expected = 2;
        let output = maximum_safeness_factor(grid);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_three() {
        let grid = vec![vec![0, 0, 0, 1], vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![1, 0, 0, 0]];
        let expected = 2;
        let output = maximum_safeness_factor(grid);
        assert_eq!(expected, output);
    }

    #[test]
    fn test_four() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
                        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
                        vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0]];
        let expected = 7;
        let output = maximum_safeness_factor(grid);
        assert_eq!(expected, output);
    }
}