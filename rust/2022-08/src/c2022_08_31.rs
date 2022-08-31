#![allow(dead_code)]
//! \#417. Pacific Atlantic Water Flow
//! ==================================
//!
//! <https://leetcode.com/problems/pacific-atlantic-water-flow>
//!
//! There is an `m x n` rectangular island that borders both the __Pacific Ocean__ and __Atlantic Ocean__.
//! The __Pacific Ocean__ touches the island's left and top edges,
//! and the __Atlantic Ocean__ touches the island's right and bottom edges.
//!
//! The island is partitioned into a grid of square cells.
//! You are given an `m x n` integer matrix `heights` where `heights[r][c]`
//! represents the __height above sea level__ of the cell at coordinate `(r, c)`.
//!
//! The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east,
//! and west if the neighboring cell's height is __less than or equal__ to the current cell's height.
//! Water can flow from any cell adjacent to an ocean into the ocean.
//!
//! Return _a __2D list__ of grid coordinates `result` where `result[i] = [ri, ci]`
//! denotes that rain water can flow from cell `(ri, ci)` to __both__ the Pacific and Atlantic oceans_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//! pacific     /
//!  +---------+
//!  |1 2 2 3 5|
//!  |3 2 3 4 4|
//!  |2 4 5 3 1|
//!  |6 7 1 4 5|
//!  |5 1 1 2 4|
//!  +---------+
//! /    atlantic
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_31::*;
//! # use c2022_08::vv;
//! let heights = vv![[1, 2, 2, 3, 5], [3, 2, 3, 4, 4], [2, 4, 5, 3, 1], [6, 7, 1, 4, 5], [5, 1, 1, 2, 4]];
//! let expected = [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]];
//! assert_eq!(Solution::pacific_atlantic(heights), expected);
//! ```
//!
//! __Explanation:__ The following cells can flow to the Pacific and Atlantic oceans, as shown below:
//! - `[0,4]`:
//!   - `[0,4]` -> Pacific Ocean
//!   - `[0,4]` -> Atlantic Ocean
//! - `[1,3]`:
//!   - `[1,3]` -> `[0,3]` -> Pacific Ocean
//!   - `[1,3]` -> `[1,4]` -> Atlantic Ocean
//! - `[1,4]`:
//!   - `[1,4]` -> `[1,3]` -> `[0,3]` -> Pacific Ocean
//!   - `[1,4]` -> Atlantic Ocean
//! - `[2,2]`:
//!   - `[2,2]` -> `[1,2]` -> `[0,2]` -> Pacific Ocean
//!   - `[2,2]` -> `[2,3]` -> `[2,4]` -> Atlantic Ocean
//! - `[3,0]`:
//!   - `[3,0]` -> Pacific Ocean
//!   - `[3,0]` -> `[4,0]` -> Atlantic Ocean
//! - `[3,1]`:
//!   - `[3,1]` -> `[3,0]` -> Pacific Ocean
//!   - `[3,1]` -> `[4,1]` -> Atlantic Ocean
//! - `[4,0]`:
//!   - `[4,0]` -> Pacific Ocean
//!   - `[4,0]` -> Atlantic Ocean
//!
//! Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_31::*;
//! # use c2022_08::vv;
//! let heights = vv![[1]];
//! assert_eq!(Solution::pacific_atlantic(heights), [[0, 0]]);
//! ```
//!
//! __Explanation:__ The water can flow from the only cell to the Pacific and Atlantic oceans.
//!
//! ##### Constraints
//!
//! - `m == heights.length`
//! - `n == heights[r].length`
//! - `1 <= m, n <= 200`
//! - `0 <= heights[r][c] <= 100_000`

pub struct Solution;
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::iter::once;

        fn go_up(r: usize, c: usize, heights: &Vec<Vec<i32>>, memo: &mut Vec<Vec<bool>>) {
            if !memo[r][c] {
                memo[r][c] = true;
                let (m, n) = (heights.len(), heights[0].len());
                once((r.wrapping_sub(1), c))
                    .chain(once((r + 1, c)))
                    .chain(once((r, c.wrapping_sub(1))))
                    .chain(once((r, c + 1)))
                    .filter(|&(r, c)| r < m && c < n)
                    .filter(|&(nr, nc)| heights[r][c] <= heights[nr][nc])
                    .for_each(|(r, c)| go_up(r, c, heights, memo));
            }
        }

        let (m, n) = (heights.len(), heights[0].len());

        let mut pacific = vec![vec![false; n]; m];
        for r in 0..m {
            go_up(r, 0, &heights, &mut pacific);
        }
        for c in 0..n {
            go_up(0, c, &heights, &mut pacific);
        }

        let mut atlantic = vec![vec![false; n]; m];
        for r in 0..m {
            go_up(r, n - 1, &heights, &mut atlantic);
        }
        for c in 0..n {
            go_up(m - 1, c, &heights, &mut atlantic);
        }

        let mut result = vec![];
        for r in 0..m {
            for c in 0..n {
                if pacific[r][c] && atlantic[r][c] {
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }
        result
    }
}
