#![allow(dead_code)]
/// \#63. Unique Paths II
/// =====================
///
/// You are given an `m x n` integer array `grid`.
/// There is a robot initially located at the __top-left__ corner (i.e., `grid[0][0]`).
/// The robot tries to move to the __bottom-right corner__ (i.e., `grid[m-1][n-1]`).
/// The robot can only move either down or right at any point in time.
///
/// An obstacle and space are marked as `1` or `0` respectively in `grid`.
/// A path that the robot takes cannot include __any__ square that is an obstacle.
///
/// Return _the number of possible unique paths that the robot can take to reach the bottom-right corner_.
///
/// The testcases are generated so that the answer will be less than or equal to `2_000_000_000`.
///
/// __Constraints:__
///
/// - `m == obstacleGrid.length`
/// - `n == obstacleGrid[i].length`
/// - `1 <= m, n <= 100`
/// - `obstacleGrid[i][j]` is `0` or `1`.
///
/// https://leetcode.com/problems/unique-paths-ii/
struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[m - 1][n - 1] == 1 {
            0
        } else {
            let mut dp = vec![0; n + 1];
            dp[n - 1] = 1;
            for r in (0..m).rev() {
                for c in (0..n).rev() {
                    if obstacle_grid[r][c] == 0 {
                        dp[c] += dp[c + 1];
                    } else {
                        dp[c] = 0;
                    }
                }
            }
            dp[0]
        }
    }
    pub fn unique_paths_with_obstacles_dp_vec_vec(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[m - 1][n - 1] == 1 {
            0
        } else {
            let mut dp = vec![vec![0; n + 1]; m + 1];
            dp[m][n - 1] = 1;
            for r in (0..m).rev() {
                for c in (0..n).rev() {
                    if obstacle_grid[r][c] == 0 {
                        dp[r][c] = dp[r][c + 1] + dp[r + 1][c];
                    }
                }
            }
            dp[0][0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[rustfmt::skip] #[test] fn g_0() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0]]), 1); }

    #[rustfmt::skip] #[test] fn g_00() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0,0]]), 1); }
    #[rustfmt::skip] #[test] fn g_0_0() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0],[0]]), 1); }

    #[rustfmt::skip] #[test] fn g_000() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0,0,0]]), 1); }
    #[rustfmt::skip] #[test] fn g_0_0_0() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0],[0],[0]]), 1); }

    #[rustfmt::skip] #[test] fn g_00_00() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0,0],[0,0]]), 2); }
    #[rustfmt::skip] #[test] fn g_01_00() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0,1],[0,0]]), 1); }

    #[test]
    fn g_000_010_000() {
        let g = vv![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(g), 2);
        // Explanation: There is one obstacle in the middle of the 3x3 grid above.
        // There are two ways to reach the bottom-right corner:
        // 1. Right -> Right -> Down -> Down
        // 2. Down -> Down -> Right -> Right
    }
    #[test]
    fn g_000_000_000() {
        let g = vv![[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(g), 6);
    }
}
