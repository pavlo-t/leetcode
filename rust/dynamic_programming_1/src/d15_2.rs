#![allow(dead_code)]
/// 63. Unique Paths II
/// ===================
///
/// A robot is located at the top-left corner of a `m x n` grid.
///
/// The robot can only move either down or right at any point in time.
/// The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
///
/// Now consider if some obstacles are added to the grids.
/// How many unique paths would there be?
///
/// An obstacle and space is marked as `1` and `0` respectively in the grid.
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
    /// 09:31-09:35
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![0; n + 1];
        dp[n - 1] = (obstacle_grid[m - 1][n - 1] == 0) as i32;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if obstacle_grid[r][c] == 0 {
                    dp[c] = dp[c].wrapping_add(dp[c + 1]);
                } else {
                    dp[c] = 0;
                }
            }
        }
        dp[0]
    }
    /// 00:06-00:12
    pub fn unique_paths_with_obstacles_dp_vec_vec(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        dp[m][n - 1] = 1;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if obstacle_grid[r][c] == 0 {
                    dp[r][c] = dp[r][c + 1].wrapping_add(dp[r + 1][c]);
                }
            }
        }
        dp[0][0]
    }
    /// 00:04-00:06
    pub fn unique_paths_with_obstacles_rec_with_memo(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        fn rec(r: usize, c: usize, og: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            let (m, n) = (og.len(), og[0].len());
            let (tr, tc) = (m - 1, n - 1);
            if r > tr || c > tc || og[r][c] == 1 {
                0
            } else if r == tr && c == tc {
                1
            } else if memo[r][c] > 0 {
                memo[r][c]
            } else {
                memo[r][c] = rec(r, c + 1, og, memo).wrapping_add(rec(r + 1, c, og, memo));
                memo[r][c]
            }
        }
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut memo = vec![vec![0; n]; m];
        rec(0, 0, &obstacle_grid, &mut memo)
    }
    /// 23:52-00:04
    pub fn unique_paths_with_obstacles_rec(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_with_obstacles({:?})", obstacle_grid);
        fn rec(r: usize, c: usize, og: &[Vec<i32>]) -> i32 {
            let (m, n) = (og.len(), og[0].len());
            let (tr, tc) = (m - 1, n - 1);
            if r > tr || c > tc || og[r][c] == 1 {
                0
            } else if r == tr && c == tc {
                1
            } else {
                rec(r, c + 1, og).wrapping_add(rec(r + 1, c, og))
            }
        }
        rec(0, 0, &obstacle_grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

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
    fn o_000_111_000() {
        let o = vv![[0, 0, 0], [1, 1, 1], [0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 0);
    }
    #[test]
    fn o_010_010_010() {
        let o = vv![[0, 1, 0], [0, 1, 0], [0, 1, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 0);
    }
    #[test]
    fn o_000_010_001() {
        let o = vv![[0, 0, 0], [0, 1, 0], [0, 0, 1]];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 0);
    }
    #[test]
    fn o_0() {
        assert_eq!(Solution::unique_paths_with_obstacles(vv![[0]]), 1);
    }
    #[test]
    fn o_1() {
        assert_eq!(Solution::unique_paths_with_obstacles(vv![[1]]), 0);
    }

    #[test]
    fn o_100x7x0() {
        let o = vec![vec![0; 7]; 100];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 1_609_344_100);
    }
    #[test]
    fn o_7x100x0() {
        let o = vec![vec![0; 100]; 7];
        assert_eq!(Solution::unique_paths_with_obstacles(o), 1_609_344_100);
    }
    #[test]
    fn o_100x100x0() {
        let o = vec![vec![0; 100]; 100];
        assert_eq!(Solution::unique_paths_with_obstacles(o), -2_129_403_600);
    }
}
