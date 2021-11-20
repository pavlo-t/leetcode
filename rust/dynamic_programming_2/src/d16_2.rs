#![allow(dead_code)]
/// 63. Unique Paths II
/// ===================
///
/// A robot is located at the top-left corner of a `m x n` grid.
///
/// The robot can only move either down or right at any point in time.
/// The robot is trying to reach the bottom-right corner of the grid.
///
/// Now consider if some obstacles are added to the grids.
/// How many unique paths would there be?
///
/// An obstacle and space is marked as `1` and `0` respectively in the grid.
///
/// __Constraints:__
///
/// - `1 <= obstacleGrid.length, obstacleGrid[i].length <= 100
/// - `obstacleGrid[i][j] is 0 or 1.
///
/// https://leetcode.com/problems/unique-paths-ii/
struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles_rec(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        fn rec(r: usize, c: usize, g: &[Vec<i32>]) -> i32 {
            let (m, n) = (g.len(), g[0].len());
            if r == m || c == n || g[r][c] == 1 {
                0
            } else if r == m - 1 && c == n - 1 {
                1
            } else {
                rec(r + 1, c, g).wrapping_add(rec(r, c + 1, g))
            }
        }
        rec(0, 0, &obstacle_grid)
    }
    pub fn unique_paths_with_obstacles_rec_with_memo(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        fn rec(r: usize, c: usize, g: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[r][c] != -1 {
                memo[r][c]
            } else {
                let (m, n) = (g.len(), g[0].len());
                memo[r][c] = if r == m || c == n || g[r][c] == 1 {
                    0
                } else if r == m - 1 && c == n - 1 {
                    1
                } else {
                    rec(r + 1, c, g, memo).wrapping_add(rec(r, c + 1, g, memo))
                };
                memo[r][c]
            }
        }
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut memo = vec![vec![-1; n + 1]; m + 1];
        rec(0, 0, &obstacle_grid, &mut memo)
    }
    pub fn unique_paths_with_obstacles_dp_vec_vec(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        dp[m][n - 1] = 1;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if obstacle_grid[r][c] == 1 {
                    dp[r][c] = 0;
                } else {
                    dp[r][c] = dp[r + 1][c].wrapping_add(dp[r][c + 1]);
                }
            }
        }
        dp[0][0]
    }
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[n - 1] = 1;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if obstacle_grid[r][c] == 1 {
                    dp[c] = 0;
                } else {
                    dp[c] = dp[c].wrapping_add(dp[c + 1]);
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[rustfmt::skip] #[test] fn o_0() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0]]), 1); }
    #[rustfmt::skip] #[test] fn o_1() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[1]]), 0); }

    #[rustfmt::skip] #[test] fn o_00() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0,0]]), 1); }
    #[rustfmt::skip] #[test] fn o_0_0() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0],[0]]), 1); }
    #[rustfmt::skip] #[test] fn o_10() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[1,0]]), 0); }
    #[rustfmt::skip] #[test] fn o_01() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0,1]]), 0); }
    #[rustfmt::skip] #[test] fn o_1_0() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[1],[0]]), 0); }
    #[rustfmt::skip] #[test] fn o_0_1() { assert_eq!(Solution::unique_paths_with_obstacles(vv![[0],[1]]), 0); }

    #[test]
    fn o_000_010_000() {
        let o = vv![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 2);
        // Explanation: There is one obstacle in the middle of the 3x3 grid above.
        // There are two ways to reach the bottom-right corner:
        // 1. Right -> Right -> Down -> Down
        // 2. Down -> Down -> Right -> Right
    }
    #[test]
    fn o_01_00() {
        let o = vv![[0, 1], [0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 1);
    }

    #[test]
    fn o_100x100x0() {
        let o = vec![vec![0; 100]; 100];
        assert_eq!(Solution::unique_paths_with_obstacles(o), -2129403600);
    }
}
