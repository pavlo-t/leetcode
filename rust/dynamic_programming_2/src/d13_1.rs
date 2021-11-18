#![allow(dead_code)]
/// 64. Minimum Path Sum
/// ====================
///
/// Given a `m x n` grid filled with non-negative numbers,
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
    pub fn min_path_sum_rec(grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        fn rec(r: usize, c: usize, g: &[Vec<i32>]) -> i32 {
            if r == g.len() || c == g[0].len() {
                0
            } else if r == g.len() - 1 {
                g[r][c] + rec(r, c + 1, g)
            } else if c == g[0].len() - 1 {
                g[r][c] + rec(r + 1, c, g)
            } else {
                g[r][c] + rec(r + 1, c, g).min(rec(r, c + 1, g))
            }
        }
        rec(0, 0, &grid)
    }
    pub fn min_path_sum_rec_with_memo(grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        fn rec(r: usize, c: usize, g: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[r][c] != -1 {
                memo[r][c]
            } else {
                memo[r][c] = if r == g.len() || c == g[0].len() {
                    0
                } else if r == g.len() - 1 {
                    g[r][c] + rec(r, c + 1, g, memo)
                } else if c == g[0].len() - 1 {
                    g[r][c] + rec(r + 1, c, g, memo)
                } else {
                    g[r][c] + rec(r + 1, c, g, memo).min(rec(r, c + 1, g, memo))
                };
                memo[r][c]
            }
        }
        let mut memo = vec![vec![-1; grid[0].len() + 1]; grid.len() + 1];
        rec(0, 0, &grid, &mut memo)
    }
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        println!("min_path_sum({:?})", grid);
        let (m, n) = (grid.len(), grid[0].len());
        for r in (0..m - 1).rev() {
            grid[r][n - 1] += grid[r + 1][n - 1];
        }
        for c in (0..n - 1).rev() {
            grid[m - 1][c] += grid[m - 1][c + 1];
        }
        for r in (0..m - 1).rev() {
            for c in (0..n - 1).rev() {
                grid[r][c] += grid[r + 1][c].min(grid[r][c + 1]);
            }
        }
        grid[0][0]
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
    fn g_1x200x200() {
        let g = vec![vec![1; 200]; 200];
        assert_eq!(Solution::min_path_sum(g), 399);
    }
}
