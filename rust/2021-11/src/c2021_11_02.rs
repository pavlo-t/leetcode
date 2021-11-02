#![allow(dead_code)]
/// 980. Unique Paths III
/// =====================
///
/// You are given an `m x n` integer array `grid` where `grid[i][j]` could be:
///
/// - `1` representing the starting square. There is exactly one starting square.
/// - `2` representing the ending square. There is exactly one ending square.
/// - `0` representing empty squares we can walk over.
/// - `-1` representing obstacles that we cannot walk over.
///
/// Return _the number of 4-directional walks from the starting square to the ending square,
/// that walk over every non-obstacle square exactly once_.
///
/// __Constraints:__
///
/// - `m == grid.length`
/// - `n == grid[i].length`
/// - `1 <= m, n <= 20`
/// - `1 <= m * n <= 20`
/// - `-1 <= grid[i][j] <= 2`
/// - There is exactly one starting cell and one ending cell.
///
/// https://leetcode.com/problems/unique-paths-iii/
struct Solution;
impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        println!("unique_paths_iii({:?})", grid);
        use std::iter::once;

        /// Top, Bottom, Left, Right
        fn tblr(r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
            once((r.wrapping_sub(1), c))
                .chain(once((r + 1, c)))
                .chain(once((r, c.wrapping_sub(1))))
                .chain(once((r, c + 1)))
        }
        fn bts(r: usize, c: usize, g: &mut Vec<Vec<i32>>, must_see: i32) -> i32 {
            if g[r][c] == 2 {
                (must_see == 0) as i32
            } else {
                g[r][c] -= 4;
                let mut result = 0;
                for (r, c) in tblr(r, c) {
                    if r < g.len() && c < g[0].len() && g[r][c] >= 0 {
                        result += bts(r, c, g, must_see - 1);
                    }
                }
                g[r][c] += 4;
                result
            }
        }

        let (m, n) = (grid.len(), grid[0].len());
        let ((start_r, start_c), must_see) = (0..m).flat_map(|r| (0..n).map(move |c| (r, c))).fold(
            ((0, 0), 0),
            |(st, ms), (r, c)| match grid[r][c] {
                0 | 2 => (st, ms + 1),
                1 => ((r, c), ms),
                _ => (st, ms),
            },
        );
        bts(start_r, start_c, &mut grid, must_see)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn g_1000_0000_002x() {
        let g = vv![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]];
        assert_eq!(Solution::unique_paths_iii(g), 2);
        // Explanation: We have the following two paths:
        // 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
        // 2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
    }
    #[test]
    fn g_1000_0000_0002() {
        let g = vv![[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]];
        assert_eq!(Solution::unique_paths_iii(g), 4);
        // Explanation: We have the following four paths:
        // 1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
        // 2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
        // 3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
        // 4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
    }
    #[test]
    fn g_01_20() {
        let g = vv![[0, 1], [2, 0]];
        assert_eq!(Solution::unique_paths_iii(g), 0);
        // Explanation: There is no path that walks over every empty square exactly once.
        // Note that the starting and ending square can be anywhere in the grid.
    }
    #[test]
    fn g_12() {
        let g = vv![[1, 2]];
        assert_eq!(Solution::unique_paths_iii(g), 1);
    }
    #[test]
    fn g_1x2() {
        let g = vv![[1, -1, 2]];
        assert_eq!(Solution::unique_paths_iii(g), 0);
    }
    #[test]
    fn g_10000_00000_00000_00002() {
        let g = vv![
            [1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2]
        ];
        assert_eq!(Solution::unique_paths_iii(g), 20);
    }
    #[test]
    fn g_12000_00000_00000_00000() {
        let g = vv![
            [1, 2, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0]
        ];
        assert_eq!(Solution::unique_paths_iii(g), 14);
    }
}
