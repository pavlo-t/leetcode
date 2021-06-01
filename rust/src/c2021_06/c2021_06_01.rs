#![allow(dead_code)]
/// Max Area of Island
/// ==================
///
/// You are given an `m x n` binary matrix `grid`.
/// An island is a group of `1`'s (representing land) connected __4-directionally__ (horizontal or vertical.)
/// You may assume all four edges of the grid are surrounded by water.
///
/// The __area__ of an island is the number of cells with a value `1` in the island.
///
/// Return _the maximum __area__ of an island in `grid`_.
/// If there is no island, return `0`.
///
/// __Constraints:__
///
/// - `m == grid.length`
/// - `n == grid[i].length`
/// - `1 <= m, n <= 50`
/// - `grid[i][j]` is either `0` or `1`.
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3764/
struct Solution;
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(r: usize, c: usize, g: &[Vec<i32>], seen: &mut [Vec<bool>]) -> i32 {
            if g[r][c] == 0 || seen[r][c] {
                0
            } else {
                seen[r][c] = true;
                let mut result = 1;
                if r > 0 {
                    result += dfs(r - 1, c, g, seen);
                }
                if r < g.len() - 1 {
                    result += dfs(r + 1, c, g, seen);
                }
                if c > 0 {
                    result += dfs(r, c - 1, g, seen);
                }
                if c < g[0].len() - 1 {
                    result += dfs(r, c + 1, g, seen);
                }
                result
            }
        }

        let mut result = 0;
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                result = result.max(dfs(r, c, &grid, &mut seen))
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        let grid = vv![
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);
        // Explanation: The answer is not 11, because the island must be connected 4-directionally.
    }
    #[test]
    fn example2() {
        let grid = vv![[0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 0);
    }

    #[test]
    fn g50x50_1s_produces_2500() {
        let grid = vec![vec![1; 50]; 50];
        assert_eq!(Solution::max_area_of_island(grid), 2500);
    }
    #[test]
    fn g50x50_0s_produces_0() {
        let grid = vec![vec![0; 50]; 50];
        assert_eq!(Solution::max_area_of_island(grid), 0);
    }
}
