#![allow(dead_code)]
/// \#52. N-Queens II
/// =================
///
/// The __n-queens__ puzzle is the problem of placing `n` queens on an `n x n` chessboard
/// such that no two queens attack each other.
///
/// Given an integer `n`, return _the number of distinct solutions to the __n-queens puzzle___.
///
/// __Constraints:__
///
/// - `1 <= n <= 9`
///
/// https://leetcode.com/problems/n-queens-ii
struct Solution;
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn bts(
            r: usize,
            c: usize,
            rs: &mut Vec<bool>,
            cs: &mut Vec<bool>,
            ds: &mut Vec<bool>,
            ads: &mut Vec<bool>,
        ) -> i32 {
            let n = rs.len();
            if r == n {
                1
            } else if c == n {
                0
            } else if rs[r] && cs[c] && ds[n + c - r - 1] && ads[r + c] {
                let mut result = 0;

                rs[r] = false;
                cs[c] = false;
                ds[n + c - r - 1] = false;
                ads[r + c] = false;

                result += bts(r + 1, 0, rs, cs, ds, ads);

                rs[r] = true;
                cs[c] = true;
                ds[n + c - r - 1] = true;
                ads[r + c] = true;

                result + bts(r, c + 1, rs, cs, ds, ads)
            } else {
                bts(r, c + 1, rs, cs, ds, ads)
            }
        }

        let n = n as usize;
        let mut rows = vec![true; n];
        let mut cols = vec![true; n];
        let diag_len = n * 2 - 1;
        let mut diagonals = vec![true; diag_len];
        let mut anti_diagonals = vec![true; diag_len];

        bts(0, 0, &mut rows, &mut cols, &mut diagonals, &mut anti_diagonals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::total_n_queens(1), 1); }
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::total_n_queens(2), 0); }
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::total_n_queens(3), 0); }
    /// ```text
    /// . . . .
    /// . . . .
    /// . . . .
    /// . . . .
    /// diagonals:
    /// len = n * 2 - 1;
    /// i = n + c - r - 1;
    /// 00 01 02 03
    /// 10 11 12 13
    /// 20 21 22 23
    /// 30 31 32 33
    ///  3  4  5  6
    ///  2  3  4  5
    ///  1  2  3  4
    ///  0  1  2  3
    /// anti-diagonals:
    /// len = n * 2 - 1;
    /// i = r + c
    /// 00 01 02 03
    /// 10 11 12 13
    /// 20 21 22 23
    /// 30 31 32 33
    ///  0  1  2  3
    ///  1  2  3  4
    ///  2  3  4  5
    ///  3  4  5  6
    /// ```
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::total_n_queens(4), 2); }
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::total_n_queens(5), 10); }
    #[rustfmt::skip] #[test] fn n_9() { assert_eq!(Solution::total_n_queens(9), 352); }
}
