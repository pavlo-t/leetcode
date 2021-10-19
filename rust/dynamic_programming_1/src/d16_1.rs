#![allow(dead_code)]
/// 64. Minimum Path Sum
/// ====================
///
/// Given a `m x n` `grid` filled with non-negative numbers,
/// find a path from top left to bottom right,
/// which minimizes the sum of all numbers along its path.
///
/// __Note:__ You can only move either down or right at any point in time.
///
/// __Constraints:__
///
/// - `1 <= grid.length, grid[i].length <= 200`
/// - `0 <= grid[i][j] <= 100`
///
/// https://leetcode.com/problems/minimum-path-sum/
struct Solution;
impl Solution {
    /// 20:30-20:37
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![i32::MAX; n + 1];
        dp[n - 1] = 0;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                dp[c] = grid[r][c] + dp[c].min(dp[c + 1]);
            }
        }
        dp[0]
    }
    /// 20:30-20:34
    pub fn min_path_sum_dp_vec_vec(grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[m][n - 1] = 0;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                dp[r][c] = grid[r][c] + dp[r + 1][c].min(dp[r][c + 1]);
            }
        }
        dp[0][0]
    }
    /// 20:28-20:30
    pub fn min_path_sum_rec_with_memo(grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        fn rec(r: usize, c: usize, g: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            let (m, n) = (g.len(), g[0].len());
            if r == m - 1 && c == n - 1 {
                g[r][c]
            } else if r >= m || c >= n {
                i32::MAX
            } else if memo[r][c] >= 0 {
                memo[r][c]
            } else {
                memo[r][c] = g[r][c] + rec(r, c + 1, g, memo).min(rec(r + 1, c, g, memo));
                memo[r][c]
            }
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut memo = vec![vec![-1; n]; m];
        rec(0, 0, &grid, &mut memo)
    }
    /// 20:23-20:28
    pub fn min_path_sum_rec(grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        fn rec(r: usize, c: usize, g: &[Vec<i32>]) -> i32 {
            let (m, n) = (g.len(), g[0].len());
            if r == m - 1 && c == n - 1 {
                g[r][c]
            } else if r >= m || c >= n {
                i32::MAX
            } else {
                g[r][c] + rec(r, c + 1, g).min(rec(r + 1, c, g))
            }
        }
        rec(0, 0, &grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn g_131_151_421() {
        let g = vv![[1, 3, 1], [1, 5, 1], [4, 2, 1]];
        assert_eq!(Solution::min_path_sum(g), 7);
        // Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
    }
    #[test]
    fn g_123_456() {
        let g = vv![[1, 2, 3], [4, 5, 6]];
        assert_eq!(Solution::min_path_sum(g), 12);
    }

    #[test]
    fn g_200x200x1() {
        let g = vec![vec![1; 200]; 200];
        assert_eq!(Solution::min_path_sum(g), 399);
    }
}
