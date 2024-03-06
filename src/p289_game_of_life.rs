#![allow(dead_code)]

pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let count = count_neighbours(row as i32, col as i32, board);

            let cur_cell_is_live = board[row][col] == 1;

            if cur_cell_is_live && (count == 2 || count == 3) {
                board[row][col] = 3;
            }

            if !cur_cell_is_live && count == 3 {
                board[row][col] = 2;
            }
        }
    }

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if board[row][col] >= 2 {
                board[row][col] = 1;
            } else {
                board[row][col] = 0;
            }
        }
    }
}

pub fn count_neighbours(row: i32, col: i32, board: &mut Vec<Vec<i32>>) -> i32 {
    let directions = vec![(-1, -1), (-1, 0), (-1, 1),
                          (0, -1), (0, 1),
                          (1, -1), (1, 0), (1, 1)];

    let mut count = 0;

    for direction in directions {
        let neighbour_row = row + direction.0;
        let neighbour_col = col + direction.1;

        if neighbour_row >= 0 && neighbour_col >= 0 && neighbour_row < board.len() as i32 && neighbour_col < board[0].len() as i32 {
            if board[neighbour_row as usize][neighbour_col as usize] % 2 == 1 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::p289_game_of_life::game_of_life;

    #[test]
    fn test_one() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        game_of_life(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn test_two() {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        let expected = vec![vec![1, 1], vec![1, 1]];
        game_of_life(&mut board);
        assert_eq!(expected, board);
    }
}