#![allow(dead_code)]
/// 994. Rotting Oranges
/// ====================
///
/// You are given an `m x n` `grid` where each cell can have one of three values:
///
/// - `0` representing an empty cell,
/// - `1` representing a fresh orange, or
/// - `2` representing a rotten orange.
///
/// Every minute, any fresh orange that is __4-directionally adjacent__ to a rotten orange becomes rotten.
///
/// Return _the minimum number of minutes that must elapse until no cell has a fresh orange_.
/// If _this is impossible, return `-1`_.
///
/// __Constraints:__
///
/// - `1 <= grid.length, grid[i].length <= 10`
/// - `grid[i][j]` is `0`, `1`, or `2`.
///
/// https://leetcode.com/problems/rotting-oranges/
struct Solution;
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        println!("oranges_rotting({:?})", grid);
        fn neighbors_to_rot(r: usize, c: usize, time: i32, grid: &mut Vec<Vec<i32>>) -> bool {
            use std::iter::once;
            let mut rotten = false;
            for (r, c) in once((r.wrapping_sub(1), c))
                .chain(once((r + 1, c)))
                .chain(once((r, c.wrapping_sub(1))))
                .chain(once((r, c + 1)))
            {
                if r < grid.len() && c < grid[0].len() && grid[r][c] == 1 {
                    rotten = true;
                    grid[r][c] = time;
                }
            }
            rotten
        }
        fn next_minute(time: i32, grid: &mut Vec<Vec<i32>>) -> bool {
            let (m, n) = (grid.len(), grid[0].len());
            let mut rotten = false;
            for r in 0..m {
                for c in 0..n {
                    if grid[r][c] == time {
                        rotten |= neighbors_to_rot(r, c, time + 1, grid);
                    }
                }
            }
            rotten
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut time = 2;
        while next_minute(time, &mut grid) {
            time += 1;
        }
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    return -1;
                }
            }
        }
        time - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn g_211_110_011() {
        let g = vv![[2, 1, 1], [1, 1, 0], [0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(g), 4);
    }
    #[test]
    fn g_211_011_101() {
        let g = vv![[2, 1, 1], [0, 1, 1], [1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(g), -1);
        // Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten,
        // because rotting only happens 4-directionally.
    }
    #[test]
    fn g_02() {
        let g = vv![[0, 2]];
        assert_eq!(Solution::oranges_rotting(g), 0);
        // Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
    }
    #[test]
    fn g_00() {
        assert_eq!(Solution::oranges_rotting(vv![[0, 0]]), 0);
    }
    #[test]
    fn g_01() {
        assert_eq!(Solution::oranges_rotting(vv![[0, 1]]), -1);
    }
    #[test]
    fn g_12() {
        assert_eq!(Solution::oranges_rotting(vv![[1, 2]]), 1);
    }

    #[test]
    fn g_211_111_012() {
        let g = vv![[2, 1, 1], [1, 1, 1], [0, 1, 2]];
        assert_eq!(Solution::oranges_rotting(g), 2);
    }
}
