#![allow(dead_code)]
/// \#51. N-Queens
/// ==============
///
/// The __n-queens__ puzzle is the problem of placing `n` queens on an `n x n` chessboard
/// such that no two queens attack each other.
///
/// Given an integer `n`, return _all distinct solutions to the __n-queens__ puzzle_.
/// You may return the answer in __any order__.
///
/// Each solution contains a distinct board configuration of the n-queens' placement,
/// where `'Q'` and `'.'` both indicate a queen and an empty space, respectively.
///
/// __Constraints:__
///
/// - `1 <= n <= 9`
///
/// https://leetcode.com/problems/n-queens
struct Solution;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        struct Taken {
            r: Vec<bool>, // rows
            c: Vec<bool>, // cols
            d: Vec<bool>, // diagonals
            a: Vec<bool>, // anti-diagonals
        }
        impl Taken {
            fn new(n: usize) -> Self {
                Self {
                    r: vec![false; n],
                    c: vec![false; n],
                    d: vec![false; n * 2 - 1],
                    a: vec![false; n * 2 - 1],
                }
            }

            fn take(&mut self, r: usize, c: usize) -> bool {
                if self.can_take(r, c) {
                    self.r[r] = true;
                    self.c[c] = true;
                    self.d[self.r.len() - 1 + r - c] = true;
                    self.a[self.r.len() * 2 - 2 - r - c] = true;
                    true
                } else {
                    false
                }
            }

            fn free(&mut self, r: usize, c: usize) {
                self.r[r] = false;
                self.c[c] = false;
                self.d[self.r.len() - 1 + r - c] = false;
                self.a[self.r.len() * 2 - 2 - r - c] = false;
            }

            fn can_take(&self, r: usize, c: usize) -> bool {
                !self.r[r]
                    && !self.c[c]
                    && !self.d[self.r.len() - 1 + r - c]
                    && !self.a[self.r.len() * 2 - 2 - r - c]
            }
        }

        const Q: u8 = b'Q';
        const E: u8 = b'.';

        fn to_vec_string(v: &Vec<Vec<u8>>) -> Vec<String> {
            let n = v.len();
            let mut result = Vec::with_capacity(n);
            unsafe {
                for row in v {
                    result.push(String::from_utf8_unchecked(row.clone()));
                }
            }
            result
        }

        fn bts(
            r: usize,
            c: usize,
            taken: &mut Taken,
            board: &mut Vec<Vec<u8>>,
            result: &mut Vec<Vec<String>>,
        ) {
            if r == board.len() {
                result.push(to_vec_string(board));
            } else if c < board.len() {
                if taken.take(r, c) {
                    board[r][c] = Q;
                    bts(r + 1, 0, taken, board, result);
                    board[r][c] = E;
                    taken.free(r, c);
                }
                bts(r, c + 1, taken, board, result);
            }
        }

        let n = n as usize;
        let mut taken = Taken::new(n);
        let mut board = vec![vec![E; n]; n];
        let mut result: Vec<Vec<String>> = vec![];

        bts(0, 0, &mut taken, &mut board, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        assert_eq!(Solution::solve_n_queens(1), [["Q"]]);
    }
    #[test]
    fn n_2() {
        let e: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::solve_n_queens(2), e);
    }
    #[test]
    fn n_3() {
        let e: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::solve_n_queens(2), e);
    }
    #[rustfmt::skip] #[test]
    fn n_4() {
        assert_eq!(Solution::solve_n_queens(4), [[".Q..",
                                                  "...Q",
                                                  "Q...",
                                                  "..Q."],
                                                 ["..Q.",
                                                  "Q...",
                                                  "...Q",
                                                  ".Q.."]]);
        // Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
        // (0,0) (0,1) (0,2) (0,3)
        // (1,0) (1,1) (1,2) (1,3)
        // (2,0) (2,1) (2,2) (2,3)
        // (3,0) (3,1) (3,2) (3,3)
        // n - 1 + r - c:           (size: n * 2 - 1)
        //   3     2     1     0
        //   4     3     2     1
        //   5     4     3     2
        //   6     5     4     3
        // n * 2 - 2 - r - c:       (size: n * 2 - 1)
        //   6     5     4     3
        //   5     4     3     2
        //   4     3     2     1
        //   3     2     1     0
    }
    #[test]
    fn n_5() {
        assert_eq!(
            Solution::solve_n_queens(5),
            [
                ["Q....", "..Q..", "....Q", ".Q...", "...Q."],
                ["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
                [".Q...", "...Q.", "Q....", "..Q..", "....Q"],
                [".Q...", "....Q", "..Q..", "Q....", "...Q."],
                ["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
                ["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
                ["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
                ["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
                ["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
                ["....Q", "..Q..", "Q....", "...Q.", ".Q..."]
            ]
        );
    }
    #[test]
    fn n_9() {
        let r = Solution::solve_n_queens(9);
        assert_eq!(r.len(), 352);
    }
}
