#![allow(dead_code)]
//! \#695. Max Area of Island
//! =========================
//!
//! <https://leetcode.com/problems/max-area-of-island>
//!
//! You are given an `m x n` binary matrix `grid`.
//! An island is a group of `1`'s (representing land) connected __4-directionally__ (horizontal or vertical).
//! You may assume all four edges of the grid are surrounded by water.
//!
//! The __area__ of an island is the number of cells with a value `1` in the island.
//!
//! Return _the maximum __area__ of an island in `grid`_.
//! If there is no island, return `0`.
//!
//! __Constraints:__
//!
//! - `m == grid.length`
//! - `n == grid[i].length`
//! - `1 <= m, n <= 50`
//! - `grid[i][j]` is either `0` or `1`.

pub struct Solution;
impl Solution {
    pub fn max_area_of_island_recursive(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(r: usize, c: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
            use std::iter::once;

            grid[r][c] = 2;
            let mut result = 1;
            for (r, c) in once((r + 1, c))
                .chain(once((r.wrapping_sub(1), c)))
                .chain(once((r, c + 1)))
                .chain(once((r, c.wrapping_sub(1))))
            {
                if r < grid.len() && c < grid[0].len() && grid[r][c] == 1 {
                    result += dfs(r, c, grid);
                }
            }
            result
        }

        let mut result = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    result = result.max(dfs(r, c, &mut grid));
                }
            }
        }
        result
    }

    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::iter::once;

        let mut result = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    grid[r][c] = 2;
                    let mut curr = 0;
                    let mut stack = vec![(r, c)];
                    while let Some((r, c)) = stack.pop() {
                        curr += 1;
                        for (r, c) in once((r + 1, c))
                            .chain(once((r.wrapping_sub(1), c)))
                            .chain(once((r, c + 1)))
                            .chain(once((r, c.wrapping_sub(1))))
                        {
                            if r < grid.len() && c < grid[0].len() && grid[r][c] == 1 {
                                grid[r][c] = 2;
                                stack.push((r, c));
                            }
                        }
                    }

                    result = result.max(curr);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn r0() {
        assert_eq!(Solution::max_area_of_island(vv![[0]]), 0);
    }
    #[test]
    fn r1() {
        assert_eq!(Solution::max_area_of_island(vv![[1]]), 1);
    }
    #[test]
    fn r11() {
        assert_eq!(Solution::max_area_of_island(vv![[1, 1]]), 2);
    }
    #[test]
    fn r11r11() {
        assert_eq!(Solution::max_area_of_island(vv![[1, 1], [1, 1]]), 4);
    }

    #[test]
    fn r00000000() {
        let g = vv![[0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(g), 0);
    }

    #[test]
    fn example_1() {
        let g = vv![
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ];
        assert_eq!(Solution::max_area_of_island(g), 6);
        // Explanation: The answer is not 11, because the island must be connected 4-directionally.
    }

    #[test]
    fn g_50x50x1() {
        assert_eq!(Solution::max_area_of_island(vec![vec![1; 50]; 50]), 2500);
    }
}
