#![allow(dead_code)]
//! \#62. Unique Paths
//! ==================
//!
//! <https://leetcode.com/problems/unique-paths>
//!
//! There is a robot on an `m x n` grid.
//! The robot is initially located at the __top-left corner__ (i.e., `grid[0][0]`).
//! The robot tries to move to the __bottom-right corner__ (i.e., `grid[m - 1][n - 1]`).
//! The robot can only move either down or right at any point in time.
//!
//! Given the two integers `m` and `n`, return
//! _the number of possible unique paths that the robot can take to reach the bottom-right corner_.
//!
//! The test cases are generated so that the answer will be less than or equal to `2_000_000_000`.
//!
//! __Constraints:__
//!
//! - `1 <= m, n <= 100`
//!
//! # Examples
//!
//! ```
//! # use c2022_08::c2022_08_01::Solution;
//! assert_eq!(Solution::unique_paths(3, 7), 28);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_01::Solution;
//! assert_eq!(Solution::unique_paths(3, 2), 3);
//! ```
//!
//! __Explanation:__ From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
//!
//! 1. Right -> Down -> Down
//! 2. Down -> Down -> Right
//! 3. Down -> Right -> Down
//!
//! ```
//! # use c2022_08::c2022_08_01::Solution;
//! assert_eq!(Solution::unique_paths(1, 1), 1);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_01::Solution;
//! assert_eq!(Solution::unique_paths(2, 1), 1);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_01::Solution;
//! assert_eq!(Solution::unique_paths(1, 2), 1);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_01::Solution;
//! assert_eq!(Solution::unique_paths(2, 2), 2);
//! ```

pub struct Solution;
impl Solution {
    /// Bruteforce recursion
    pub fn unique_paths_v1(m: i32, n: i32) -> i32 {
        fn rec(m: usize, n: usize) -> i32 {
            if m == 0 || n == 0 {
                1
            } else {
                rec(m, n - 1) + rec(m - 1, n)
            }
        }
        rec(m as usize - 1, n as usize - 1)
    }

    /// Recursion with memo
    pub fn unique_paths_v2(m: i32, n: i32) -> i32 {
        fn rec(m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[m][n] == -1 {
                memo[m][n] = if m == 0 || n == 0 {
                    1
                } else {
                    rec(m, n - 1, memo) + rec(m - 1, n, memo)
                };
            }
            memo[m][n]
        }

        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![-1; n]; m];
        rec(m - 1, n - 1, &mut memo)
    }

    /// DP 2 dimensions
    pub fn unique_paths_v3(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n]; m];
        for r in (0..m - 1).rev() {
            for c in (0..n - 1).rev() {
                dp[r][c] = dp[r][c + 1] + dp[r + 1][c];
            }
        }

        dp[0][0]
    }

    /// DP 1 dimension
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = {
            let (m, n) = (m as usize, n as usize);
            (m.max(n), m.min(n))
        };
        let mut dp = vec![1; n];
        for _ in 0..m - 1 {
            for c in (0..n - 1).rev() {
                dp[c] += dp[c + 1];
            }
        }

        dp[0]
    }
}
