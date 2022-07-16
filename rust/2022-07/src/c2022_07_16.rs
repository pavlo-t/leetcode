#![allow(dead_code)]
//! \#576. Out of Boundary Paths
//! ============================
//!
//! <https://leetcode.com/problems/out-of-boundary-paths>
//!
//! There is an `m x n` grid with a ball.
//! The ball is initially at the position `[startRow, startColumn]`.
//! You are allowed to move the ball to one of the four adjacent cells in the grid
//! (possibly out of the grid crossing the grid boundary).
//! You can apply __at most__ `maxMove` moves to the ball.
//!
//! Given the five integers `m`, `n`, `maxMove`, `startRow`, `startColumn`,
//! return _the number of paths to move the ball out of the grid boundary_.
//! Since the answer can be very large, return it __modulo__ `1_000_000_007`.
//!
//! __Constraints:__
//!
//! - `1 <= m, n <= 50`
//! - `0 <= maxMove <= 50`
//! - `0 <= startRow < m`
//! - `0 <= startColumn < n`

pub struct Solution;
impl Solution {
    /// Recursion
    pub fn find_paths_v1(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        println!("find_paths({m}, {n}, {max_move}, {start_row}, {start_column})");
        use std::iter::once;

        const MOD: i32 = 1_000_000_007;

        fn rec(moves: usize, r: usize, c: usize, m: usize, n: usize) -> i32 {
            if r >= m || c >= n {
                1
            } else if moves == 0 {
                0
            } else {
                let mut result = 0;
                for (r, c) in once((r + 1, c))
                    .chain(once((r.wrapping_sub(1), c)))
                    .chain(once((r, c + 1)))
                    .chain(once((r, c.wrapping_sub(1))))
                {
                    result += rec(moves - 1, r, c, m, n) % MOD
                }
                result % MOD
            }
        }
        let (moves, r, c) = (max_move as usize, start_row as usize, start_column as usize);
        let (m, n) = (m as usize, n as usize);

        rec(moves, r, c, m, n)
    }

    /// Recursion with memo
    pub fn find_paths_v2(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        println!("find_paths({m}, {n}, {max_move}, {start_row}, {start_column})");
        use std::iter::once;

        const MOD: i32 = 1_000_000_007;

        #[rustfmt::skip]
        fn rec(moves: usize, r: usize, c: usize, m: usize, n: usize, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if r >= m || c >= n {
                1
            } else {
                if memo[moves][r][c] == -1 {
                    memo[moves][r][c] = if moves == 0 {
                        0
                    } else {
                        let mut result = 0;
                        for (r, c) in once((r + 1, c))
                            .chain(once((r.wrapping_sub(1), c)))
                            .chain(once((r, c + 1)))
                            .chain(once((r, c.wrapping_sub(1))))
                        {
                            result = (result + rec(moves - 1, r, c, m, n, memo)) % MOD
                        }
                        result
                    }
                };
                memo[moves][r][c]
            }
        }

        let (moves, r, c) = (max_move as usize, start_row as usize, start_column as usize);
        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![vec![-1; n]; m]; moves + 1];

        rec(moves, r, c, m, n, &mut memo)
    }

    /// DP 3 dimensions
    pub fn find_paths_v3(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let (moves, r, c) = (max_move as usize, start_row as usize, start_column as usize);
        let (m, n) = (m as usize, n as usize);

        let mut dp = vec![vec![vec![1; n + 2]; m + 2]; moves + 1];
        for r in 1..=m {
            for c in 1..=n {
                dp[0][r][c] = 0;
            }
        }
        for moves in 1..=moves {
            for r in 1..=m {
                for c in 1..=n {
                    let mut result = 0;
                    let next_moves = moves - 1;
                    result = (result + dp[next_moves][r - 1][c]) % MOD;
                    result = (result + dp[next_moves][r + 1][c]) % MOD;
                    result = (result + dp[next_moves][r][c - 1]) % MOD;
                    result = (result + dp[next_moves][r][c + 1]) % MOD;
                    dp[moves][r][c] = result;
                }
            }
        }

        dp[moves][r + 1][c + 1]
    }

    /// DP 2 dimensions
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let (moves, r, c) = (max_move as usize, start_row as usize, start_column as usize);
        let (m, n) = (m as usize, n as usize);

        let mut curr = vec![vec![1; n + 2]; m + 2];
        let mut prev = vec![vec![1; n + 2]; m + 2];
        for r in 1..=m {
            for c in 1..=n {
                prev[r][c] = 0;
            }
        }
        for _ in 0..moves {
            for r in 1..=m {
                for c in 1..=n {
                    let mut result = 0;
                    result = (result + prev[r - 1][c]) % MOD;
                    result = (result + prev[r + 1][c]) % MOD;
                    result = (result + prev[r][c - 1]) % MOD;
                    result = (result + prev[r][c + 1]) % MOD;
                    curr[r][c] = result;
                }
            }
            std::mem::swap(&mut curr, &mut prev);
        }

        prev[r + 1][c + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m1_n1_max_move_1_start_row_0_start_column_0() {
        assert_eq!(Solution::find_paths(1, 1, 1, 0, 0), 4);
    }
    #[test]
    fn m1_n1_max_move_2_start_row_0_start_column_0() {
        assert_eq!(Solution::find_paths(1, 1, 2, 0, 0), 4);
    }

    #[test]
    fn m2_n2_max_move_2_start_row_0_start_column_0() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    }
    #[test]
    fn m1_n3_max_move_3_start_row_0_start_column_1() {
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }

    #[test]
    fn m50_n50_max_move_3_start_row_0_start_column_0() {
        assert_eq!(Solution::find_paths(50, 50, 3, 0, 0), 10);
    }
    #[test]
    fn m50_n50_max_move_50_start_row_0_start_column_0() {
        assert_eq!(Solution::find_paths(50, 50, 50, 0, 0), 678_188_903);
    }
}
