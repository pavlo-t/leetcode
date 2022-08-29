#![allow(dead_code)]
//! \#200. Number of Islands
//! ========================
//!
//! <https://leetcode.com/problems/number-of-islands>
//!
//! Given an `m x n` 2D binary grid grid which represents a map of `'1'`s (land) and `'0'`s (water),
//! return _the number of islands_.
//!
//! An __island__ is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
//! You may assume all four edges of the grid are all surrounded by water.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```
//! # use c2022_08::c2022_08_29::*;
//! # use c2022_08::vv;
//! let grid = vv![
//!   ['1','1','1','1','0'],
//!   ['1','1','0','1','0'],
//!   ['1','1','0','0','0'],
//!   ['0','0','0','0','0']
//! ];
//! assert_eq!(Solution::num_islands(grid), 1);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_29::*;
//! # use c2022_08::vv;
//! let grid = vv![
//!   ['1','1','0','0','0'],
//!   ['1','1','0','0','0'],
//!   ['0','0','1','0','0'],
//!   ['0','0','0','1','1']
//! ];
//! assert_eq!(Solution::num_islands(grid), 3);
//! ```
//!
//! ##### Constraints
//!
//! - `m == grid.length`
//! - `n == grid[i].length`
//! - `1 <= m, n <= 300`
//! - `grid[i][j]` is `'0'` or `'1'`.

pub struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        use std::iter::once;

        fn adjacent((r, c): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
            once((r.wrapping_sub(1), c))
                .chain(once((r + 1, c)))
                .chain(once((r, c.wrapping_sub(1))))
                .chain(once((r, c + 1)))
        }

        fn travel_an_island_if_have_not(
            rc @ (r, c): (usize, usize),
            visited: &mut Vec<Vec<bool>>,
            grid: &Vec<Vec<char>>,
        ) -> i32 {
            if visited[r][c] || grid[r][c] != '1' {
                0
            } else {
                visited[r][c] = true;
                let (m, n) = (grid.len(), grid[0].len());
                let mut stack = vec![rc];
                while let Some(rc) = stack.pop() {
                    for rc @ (r, c) in adjacent(rc) {
                        if r < m && c < n && grid[r][c] == '1' && !visited[r][c] {
                            visited[r][c] = true;
                            stack.push(rc);
                        }
                    }
                }

                1
            }
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut distinct_islands = 0;
        let mut visited = vec![vec![false; n]; m];
        for r in 0..m {
            for c in 0..n {
                distinct_islands += travel_an_island_if_have_not((r, c), &mut visited, &grid);
            }
        }
        distinct_islands
    }
}
