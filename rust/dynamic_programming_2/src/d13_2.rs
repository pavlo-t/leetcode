#![allow(dead_code)]
/// 562. Longest Line of Consecutive One in Matrix
/// ==============================================
///
/// Given an `m x n` binary matrix `mat`,
/// return _the length of the longest line of consecutive one in the matrix_.
///
/// The line could be horizontal, vertical, diagonal, or anti-diagonal.
///
/// __Constraints:__
///
/// - `m == mat.length`
/// - `n == mat[i].length`
/// - `1 <= m, n <= 10_000`
/// - `1 <= m * n <= 10_000`
/// - `mat[i][j]` is either `0` or `1`.
///
/// https://leetcode.com/problems/longest-line-of-consecutive-one-in-matrix/
struct Solution;
impl Solution {
    pub fn longest_line_rec(mat: Vec<Vec<i32>>) -> i32 {
        println!("longest_line({:?})", mat);
        /// d: direction; 0 - right, 1 - bottom, 2 - diagonal; 3 - anti-diagonal
        fn rec(r: usize, c: usize, d: usize, m: &[Vec<i32>]) -> i32 {
            if r >= m.len() || c >= m[0].len() || m[r][c] == 0 {
                0
            } else {
                1 + match d {
                    0 => rec(r, c + 1, d, m),
                    1 => rec(r + 1, c, d, m),
                    2 => rec(r + 1, c + 1, d, m),
                    _ => rec(r + 1, c.wrapping_sub(1), d, m),
                }
            }
        }
        let (m, n) = (mat.len(), mat[0].len());
        let mut max = 0;
        for r in 0..m {
            for c in 0..n {
                for d in 0..4 {
                    max = max.max(rec(r, c, d, &mat));
                }
            }
        }
        max
    }
    pub fn longest_line_rec_with_memo(mat: Vec<Vec<i32>>) -> i32 {
        println!("longest_line({:?})", mat);
        /// d: direction; 0 - right, 1 - bottom, 2 - diagonal, 3 - anti-diagonal
        fn rec(r: usize, c: usize, d: usize, m: &[Vec<i32>], memo: &mut Vec<Vec<[i32; 4]>>) -> i32 {
            if r >= m.len() || c >= m[0].len() || m[r][c] == 0 {
                0
            } else if memo[r][c][d] != -1 {
                memo[r][c][d]
            } else {
                memo[r][c][d] = 1 + match d {
                    0 => rec(r, c + 1, d, m, memo),
                    1 => rec(r + 1, c, d, m, memo),
                    2 => rec(r + 1, c + 1, d, m, memo),
                    _ => rec(r + 1, c.wrapping_sub(1), d, m, memo),
                };
                memo[r][c][d]
            }
        }
        let (m, n) = (mat.len(), mat[0].len());
        let mut memo = vec![vec![[-1; 4]; n]; m];
        let mut max = 0;
        for r in 0..m {
            for c in 0..n {
                for d in 0..4 {
                    max = max.max(rec(r, c, d, &mat, &mut memo));
                }
            }
        }
        max
    }
    pub fn longest_line_dp_vec_vec(mat: Vec<Vec<i32>>) -> i32 {
        println!("longest_line({:?})", mat);
        let (m, n) = (mat.len(), mat[0].len());
        let mut dp = vec![vec![[0; 4]; n + 1]; m + 1];
        let mut max = 0;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if mat[r][c] == 1 {
                    dp[r][c][0] = 1 + dp[r][c + 1][0];
                    dp[r][c][1] = 1 + dp[r + 1][c][1];
                    dp[r][c][2] = 1 + dp[r + 1][c + 1][2];
                    dp[r][c][3] = 1 + if c > 0 { dp[r + 1][c - 1][3] } else { 0 };
                    max = max.max(dp[r][c].iter().max().unwrap().to_owned());
                }
            }
        }
        max
    }
    pub fn longest_line(mat: Vec<Vec<i32>>) -> i32 {
        println!("longest_line({:?})", mat);
        let (m, n) = (mat.len(), mat[0].len());
        let mut dp = vec![[0; 4]; n + 1];
        let mut max = 0;
        for r in (0..m).rev() {
            let mut prev = 0;
            for c in (0..n).rev() {
                let curr = dp[c][2];
                if mat[r][c] == 1 {
                    dp[c][0] = 1 + dp[c + 1][0];
                    dp[c][1] = 1 + dp[c][1];
                    dp[c][2] = 1 + prev;
                    dp[c][3] = 1 + if c > 0 { dp[c - 1][3] } else { 0 };
                    max = max.max(dp[c].iter().max().unwrap().to_owned());
                } else {
                    dp[c] = [0; 4];
                }
                prev = curr;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn m_0() {
        assert_eq!(Solution::longest_line(vv![[0]]), 0);
    }
    #[test]
    fn m_1() {
        assert_eq!(Solution::longest_line(vv![[1]]), 1);
    }
    #[test]
    fn m_0110_0110_0001() {
        let m = vv![[0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 1]];
        assert_eq!(Solution::longest_line(m), 3);
    }
    #[test]
    fn m_1111_0110_0001() {
        let m = vv![[1, 1, 1, 1], [0, 1, 1, 0], [0, 0, 0, 1]];
        assert_eq!(Solution::longest_line(m), 4);
    }

    #[test]
    fn test_52() {
        let m = vv![
            [1, 1, 0, 0, 1, 0, 0, 1, 1, 0],
            [1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
            [1, 1, 1, 0, 0, 1, 1, 1, 1, 0],
            [0, 1, 1, 1, 0, 1, 1, 1, 1, 1],
            [0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            [1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
            [0, 1, 1, 1, 1, 1, 1, 0, 0, 1],
            [1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
            [0, 1, 0, 1, 1, 0, 1, 1, 1, 1],
            [1, 1, 1, 0, 1, 0, 1, 1, 1, 1]
        ];
        assert_eq!(Solution::longest_line(m), 9);
    }

    #[test]
    fn m_0x100x100() {
        assert_eq!(Solution::longest_line(vec![vec![0; 100]; 100]), 0);
    }
    #[test]
    fn m_1x100x100() {
        assert_eq!(Solution::longest_line(vec![vec![1; 100]; 100]), 100);
    }
}
