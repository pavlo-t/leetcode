#![allow(dead_code)]
/// 694. Number of Distinct Islands
/// ===============================
///
/// You are given an `m x n` binary matrix `grid`.
/// An island is a group of `1`'s (representing land) connected __4-directionally__ (horizontal or vertical.)
/// You may assume all four edges of the grid are surrounded by water.
///
/// An island is considered to be the same as another if and only if one island can be translated
/// (and not rotated or reflected) to equal the other.
///
/// Return _the number of __distinct__ islands_.
///
/// __Constraints:__
///
/// - `m == grid.length`
/// - `n == grid[i].length`
/// - `1 <= m, n <= 50`
/// - `grid[i][j]` is either `0` or `1`.
///
/// https://leetcode.com/problems/number-of-distinct-islands/
struct Solution;
impl Solution {
    pub fn num_distinct_islands(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        use std::iter::once;

        fn get_island_shape(r: usize, c: usize, grid: &mut Vec<Vec<i32>>) -> Vec<(i32, i32)> {
            let (start_r, start_c) = (r as i32, c as i32);
            let (m, n) = (grid.len(), grid[0].len());
            let mut result = Vec::new();
            let mut stack = vec![(r, c)];
            while let Some((r, c)) = stack.pop() {
                result.push((start_r - r as i32, start_c - c as i32));
                for (r, c) in once((r.wrapping_sub(1), c))
                    .chain(once((r + 1, c)))
                    .chain(once((r, c.wrapping_sub(1))))
                    .chain(once((r, c + 1)))
                {
                    if r < m && c < n && grid[r][c] == 1 {
                        stack.push((r, c));
                        grid[r][c] = 2;
                    }
                }
            }
            result
        }

        let mut shapes = HashSet::new();
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    shapes.insert(get_island_shape(r, c, &mut grid));
                }
            }
        }

        shapes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[rustfmt::skip] #[test] fn g_000_000_000() { assert_eq!(Solution::num_distinct_islands(vv![[0,0,0],[0,0,0],[0,0,0]]), 0); }
    #[rustfmt::skip] #[test] fn g_101_000_101() { assert_eq!(Solution::num_distinct_islands(vv![[1,0,1],[0,0,0],[1,0,1]]), 1); }
    #[rustfmt::skip] #[test] fn g_101_010_101() { assert_eq!(Solution::num_distinct_islands(vv![[1,0,1],[0,1,0],[1,0,1]]), 1); }
    #[rustfmt::skip] #[test] fn g_101_100_101() { assert_eq!(Solution::num_distinct_islands(vv![[1,0,1],[1,0,0],[1,0,1]]), 2); }

    #[test]
    fn g_11000_11000_00011_00011() {
        let g = vv![
            [1, 1, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [0, 0, 0, 1, 1],
            [0, 0, 0, 1, 1]
        ];
        assert_eq!(Solution::num_distinct_islands(g), 1);
    }
    #[test]
    fn g_11011_10000_00001_11011() {
        let g = vv![
            [1, 1, 0, 1, 1],
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1],
            [1, 1, 0, 1, 1]
        ];
        assert_eq!(Solution::num_distinct_islands(g), 3);
    }

    #[test]
    fn g_50x50x1() {
        let g = vec![vec![1; 50]; 50];
        assert_eq!(Solution::num_distinct_islands(g), 1);
    }
}
