#![allow(dead_code)]
/// 130. Surrounded Regions
/// =======================
///
/// Given an `m x n` matrix `board` containing `'X'` and `'O'`,
/// _capture all regions that are 4-directionally surrounded by `'X'`_.
///
/// A region is __captured__ by flipping all `'O'`s into `'X'`s in that surrounded region.
///
/// __Constraints:__
///
/// - `1 <= board.length, board[i].length <= 200`
/// - `board[i][j]` is `'X'` or `'O'`.
///
/// https://leetcode.com/problems/surrounded-regions/
struct Solution;
impl Solution {
    /// Approach 1: DFS (Depth-First Search)
    /// https://leetcode.com/problems/surrounded-regions/solution/
    pub fn solve(board: &mut Vec<Vec<char>>) {
        println!("solve({:?})", board);
        use std::iter::once;
        #[rustfmt::skip]
        fn mark_outlying((r, c): (usize, usize), board: &mut Vec<Vec<char>>) {
            if board[r][c] == 'O' {
                let (m, n) = (board.len(), board[0].len());
                board[r][c] = 'E';
                let mut stack = vec![(r, c)];
                while let Some((r, c)) = stack.pop() {
                    for (r, c) in once((r.wrapping_sub(1), c))
                           .chain(once((r + 1, c)))
                           .chain(once((r, c.wrapping_sub(1))))
                           .chain(once((r, c + 1))) {
                        if r < m && c < n && board[r][c] == 'O' {
                            board[r][c] = 'E';
                            stack.push((r, c));
                        }
                    }
                }
            }
        }

        let (m, n) = (board.len(), board[0].len());
        for r in 0..m {
            mark_outlying((r, 0), board);
            mark_outlying((r, n - 1), board);
        }
        for c in 1..n - 1 {
            mark_outlying((0, c), board);
            mark_outlying((m - 1, c), board);
        }
        for r in 0..m {
            for c in 0..n {
                board[r][c] = match board[r][c] {
                    'E' => 'O',
                    _ => 'X',
                }
            }
        }
    }
    /// 20:43-21:16
    pub fn solve_my(board: &mut Vec<Vec<char>>) {
        println!("solve({:?})", board);
        use std::iter::once;
        #[rustfmt::skip]
        fn mark_outlying((r, c): (usize, usize), surrounded: &mut Vec<Vec<bool>>, board: &[Vec<char>]) {
            if surrounded[r][c] && board[r][c] == 'O' {
                surrounded[r][c] = false;
                let mut stack = vec![(r, c)];
                while let Some((r, c)) = stack.pop() {
                    for (r, c) in once((r.wrapping_sub(1), c))
                           .chain(once((r + 1, c)))
                           .chain(once((r, c.wrapping_sub(1))))
                           .chain(once((r, c + 1))) {
                        if r < board.len() && c < board[0].len() && surrounded[r][c] && board[r][c] == 'O' {
                            surrounded[r][c] = false;
                            stack.push((r, c));
                        }
                    }
                }
            }
        }

        let (m, n) = (board.len(), board[0].len());
        let mut surrounded = vec![vec![true; n]; m];
        for r in 0..m {
            mark_outlying((r, 0), &mut surrounded, &board);
            mark_outlying((r, n - 1), &mut surrounded, &board);
        }
        for c in 1..n - 1 {
            mark_outlying((0, c), &mut surrounded, &board);
            mark_outlying((m - 1, c), &mut surrounded, &board);
        }
        for r in 0..m {
            for c in 0..n {
                if surrounded[r][c] {
                    board[r][c] = 'X';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn xxxx_xoox_xxox_xoxx() {
        let mut b = vv![
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'O', 'X'],
            ['X', 'X', 'O', 'X'],
            ['X', 'O', 'X', 'X']
        ];
        let e = [
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'X', 'X', 'X'],
            ['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut b);
        assert_eq!(b, e);
        // Explanation: Surrounded regions should not be on the border,
        // which means that any 'O' on the border of the board are not flipped to 'X'.
        // Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'.
        // Two cells are connected if they are adjacent cells connected horizontally or vertically.
    }
    #[test]
    fn x() {
        let mut b = vv![['X']];
        let e = [['X']];
        Solution::solve(&mut b);
        assert_eq!(b, e);
    }
    #[test]
    fn oooo_oooo_oooo_oooo() {
        let mut b = vv![
            ['O', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O']
        ];
        let e = [
            ['O', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O'],
            ['O', 'O', 'O', 'O'],
        ];
        Solution::solve(&mut b);
        assert_eq!(b, e);
    }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib c2021_11_01
    #[test]
    fn b_200x200_o() {
        let mut b = vec![vec!['O'; 200]; 200];
        let e = vec![vec!['O'; 200]; 200];
        Solution::solve(&mut b);
        assert_eq!(b, e);
    }
    #[test]
    fn b_200x200_x() {
        let mut b = vec![vec!['X'; 200]; 200];
        let e = vec![vec!['X'; 200]; 200];
        Solution::solve(&mut b);
        assert_eq!(b, e);
    }
    #[test]
    fn b_198x198_o_surrounded_by_x() {
        let mut b = vec![vec!['O'; 200]; 200];
        for r in 0..200 {
            b[r][0] = 'X';
            b[r][199] = 'X';
        }
        for c in 1..199 {
            b[0][c] = 'X';
            b[199][c] = 'X';
        }
        let e = vec![vec!['X'; 200]; 200];
        Solution::solve(&mut b);
        assert_eq!(b, e);
    }
}
