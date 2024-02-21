#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let map = build_map(&board);
    let word: Vec<char> = word.chars().collect();
    match map.get(&word[0]) {
        Some(possible_starts) => {
            for start in possible_starts {
                let mut seen = HashSet::new();
                if backtracking(start.0, start.1, &board, &word, 0, &mut seen) {
                    return true;
                }
            }
            false
        }
        None => false
    }
}


fn build_map(board: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut map = HashMap::new();

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            (*map.entry(board[i][j]).or_insert(vec![])).push((i as i32, j as i32));
        }
    }

    map
}

fn backtracking(board_row: i32, board_col: i32, board: &Vec<Vec<char>>, word: &Vec<char>, cur_char: usize, seen: &mut HashSet<(i32, i32)>) -> bool {
    if cur_char == word.len() { return true; }

    if board_row < 0 || board_row > (board.len() - 1) as i32 || board_col < 0 || board_col > (board[0].len() - 1) as i32 {
        return false;
    }

    if seen.contains(&(board_row, board_col)) {
        return false;
    }

    if board[board_row as usize][board_col as usize] == word[cur_char] {
        seen.insert((board_row, board_col));
        let left = backtracking(board_row, board_col - 1, board, word, cur_char + 1, seen);
        let right = backtracking(board_row, board_col + 1, board, word, cur_char + 1, seen);
        let up = backtracking(board_row - 1, board_col, board, word, cur_char + 1, seen);
        let down = backtracking(board_row + 1, board_col, board, word, cur_char + 1, seen);
        seen.remove(&(board_row, board_col));

        left || right || up || down
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::word_search::exist;

    #[test]
    fn test_one() {
        assert!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("ABCCED")))
    }

    #[test]
    fn test_two() {
        assert!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("SEE")))
    }

    #[test]
    #[should_panic]
    fn test_three() {
        assert!(exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], String::from("ABCB")))
    }
}