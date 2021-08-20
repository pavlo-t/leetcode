#![allow(dead_code)]
/// Valid Sudoku
/// ============
///
/// Determine if a `9 x 9` Sudoku board is valid.
/// Only the filled cells need to be validated __according to the following rules__:
///
/// 1. Each row must contain the digits `1-9` without repetition.
/// 2. Each column must contain the digits `1-9` without repetition.
/// 3. Each of the nine `3 x 3` sub-boxes of the grid must contain the digits `1-9` without repetition.
///
/// __Note:__
///
/// - A Sudoku board (partially filled) could be valid but is not necessarily solvable.
/// - Only the filled cells need to be validated according to the mentioned rules.
///
/// __Constraints:__
///
/// - `board.length == 9`
/// - `board[i].length == 9`
/// - `board[i][j]` is a digit or `'.'`.
///
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3904/
struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        fn valid_cell(c: char, seen: i32) -> Option<i32> {
            match c.to_digit(10).map(|i| 1 << i).map(|b| (b & seen, b | seen)) {
                None => Some(seen),
                Some((0, ns)) => Some(ns),
                Some(_) => None,
            }
        }
        for r in 0..9 {
            let mut seen = 0;
            for c in 0..9 {
                match valid_cell(board[r][c], seen) {
                    Some(s) => seen = s,
                    None => return false,
                }
            }
        }
        for c in 0..9 {
            let mut seen = 0;
            for r in 0..9 {
                match valid_cell(board[r][c], seen) {
                    Some(s) => seen = s,
                    None => return false,
                }
            }
        }
        for br in 0..3 {
            for bc in 0..3 {
                let mut seen = 0;
                for r in br * 3..br * 3 + 3 {
                    for c in bc * 3..bc * 3 + 3 {
                        match valid_cell(board[r][c], seen) {
                            Some(s) => seen = s,
                            None => return false,
                        }
                    }
                }
            }
        }
        true
    }
    pub fn is_valid_sudoku_my_1(board: Vec<Vec<char>>) -> bool {
        fn valid_cell(c: char, seen: &mut Vec<bool>) -> bool {
            for d in c.to_digit(10).iter().map(|&i| i as usize) {
                if seen[d] {
                    return false;
                } else {
                    seen[d] = true;
                }
            }
            true
        }
        for r in 0..9 {
            let mut seen = vec![false; 10];
            for c in 0..9 {
                if !valid_cell(board[r][c], &mut seen) {
                    return false;
                }
            }
        }
        for c in 0..9 {
            let mut seen = vec![false; 10];
            for r in 0..9 {
                if !valid_cell(board[r][c], &mut seen) {
                    return false;
                }
            }
        }
        for br in 0..3 {
            for bc in 0..3 {
                let mut seen = vec![false; 10];
                for r in br * 3..br * 3 + 3 {
                    for c in bc * 3..bc * 3 + 3 {
                        if !valid_cell(board[r][c], &mut seen) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1_valid() {
        let board = vv![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert!(Solution::is_valid_sudoku(board));
    }
    #[test]
    fn example2_invalid() {
        let board = vv![
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert!(!Solution::is_valid_sudoku(board));
        // Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8.
        // Since there are two 8's in the top left 3x3 sub-box, it is invalid.
    }
    #[test]
    fn invalid_row_7() {
        let board = vv![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '2', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
    #[test]
    fn invalid_column_5() {
        let board = vv![
            ['5', '3', '.', '.', '8', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
    #[test]
    fn invalid_block_3_3() {
        let board = vv![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '5', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
