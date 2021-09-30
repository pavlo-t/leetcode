#![allow(dead_code)]
/// Making A Large Island
/// =====================
///
/// You are given an `n x n` binary matrix `grid`. You are allowed to change __at most one__ `0` to be `1`.
///
/// Return _the size of the largest __island__ in `grid` after applying this operation_.
///
/// An __island__ is a 4-directionally connected group of `1`s.
///
/// __Constraints:__
///
/// - `n == grid.length`
/// - `n == grid[i].length`
/// - `1 <= n <= 500`
/// - `grid[i][j]` is either `0` or `1`.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3835/
struct Solution;
impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        fn island_area(r: usize, c: usize, grid: &[Vec<i32>], seen: &mut Vec<Vec<bool>>) -> i32 {
            if grid[r][c] == 0 || seen[r][c] {
                0
            } else {
                seen[r][c] = true;
                let mut result = 1;
                if let Some(pr) = r.checked_sub(1) {
                    result += island_area(pr, c, grid, seen);
                }
                if let Some(pc) = c.checked_sub(1) {
                    result += island_area(r, pc, grid, seen);
                }
                if r < grid.len() - 1 {
                    result += island_area(r + 1, c, grid, seen);
                }
                if c < grid[r].len() - 1 {
                    result += island_area(r, c + 1, grid, seen);
                }
                result
            }
        }
        fn mark_area(r: usize, c: usize, mark: i32, areas: &mut Vec<Vec<i32>>) {
            if  areas[r][c] != 0 && areas[r][c] != mark {
                areas[r][c] = mark;
                if let Some(pr) = r.checked_sub(1) {
                    mark_area(pr, c, mark, areas);
                }
                if let Some(pc) = c.checked_sub(1) {
                    mark_area(r, pc, mark, areas);
                }
                if r < areas.len() - 1 {
                    mark_area(r + 1, c, mark, areas);
                }
                if c < areas[r].len() - 1 {
                    mark_area(r, c + 1, mark, areas);
                }
            }
        }
        let rows = grid.len();
        let cols = grid[0].len();
        let mut seen = vec![vec![false; cols]; rows];
        let mut result = 1;
        let mut island_n = 0;
        let mut areas = vec![0];
        for r in 0..rows {
            for c in 0..cols {
                if !seen[r][c] {
                    let area = island_area(r, c, &grid, &mut seen);
                    if area > 0 {
                        island_n += 1;
                        mark_area(r, c, island_n, &mut grid);
                        result = result.max(area);
                        areas.push(area);
                    }
                }
            }
        }

        println!("grid:{:?}", grid);
        println!("areas:{:?}", areas);
        let mut joined_islands = HashSet::with_capacity(4);
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 {
                    joined_islands.clear();
                    if let Some(pr) = r.checked_sub(1) {
                        joined_islands.insert(grid[pr][c] as usize);
                    }
                    if let Some(pc) = c.checked_sub(1) {
                        joined_islands.insert(grid[r][pc] as usize);
                    }
                    if r < grid.len() - 1 {
                        joined_islands.insert(grid[r + 1][c] as usize);
                    }
                    if c < grid[r].len() - 1 {
                        joined_islands.insert(grid[r][c + 1] as usize);
                    }
                    let n_res = 1 + joined_islands.iter().map(|&i| areas[i]).sum::<i32>();
                    result = result.max(n_res);
                }
            }
        }
        result
    }

    pub fn largest_island_v0_bad_counts_same_island_twice(grid: Vec<Vec<i32>>) -> i32 {
        fn island_area(r: usize, c: usize, grid: &[Vec<i32>], seen: &mut Vec<Vec<bool>>) -> i32 {
            if grid[r][c] == 0 || seen[r][c] {
                0
            } else {
                seen[r][c] = true;
                let mut result = 1;
                if let Some(pr) = r.checked_sub(1) {
                    result += island_area(pr, c, grid, seen);
                }
                if let Some(pc) = c.checked_sub(1) {
                    result += island_area(r, pc, grid, seen);
                }
                if r < grid.len() - 1 {
                    result += island_area(r + 1, c, grid, seen);
                }
                if c < grid[r].len() - 1 {
                    result += island_area(r, c + 1, grid, seen);
                }
                result
            }
        }
        fn set_area(r: usize, c: usize, area: i32, areas: &mut Vec<Vec<i32>>) {
            if  areas[r][c] != 0 && areas[r][c] != area {
                areas[r][c] = area;
                if let Some(pr) = r.checked_sub(1) {
                    set_area(pr, c, area, areas);
                }
                if let Some(pc) = c.checked_sub(1) {
                    set_area(r, pc, area, areas);
                }
                if r < areas.len() - 1 {
                    set_area(r + 1, c, area, areas);
                }
                if c < areas[r].len() - 1 {
                    set_area(r, c + 1, area, areas);
                }
            }
        }
        let rows = grid.len();
        let cols = grid[0].len();
        let mut areas = grid.clone();
        let mut seen = vec![vec![false; cols]; rows];
        let mut result = 1;
        for r in 0..rows {
            for c in 0..cols {
                if !seen[r][c] {
                    let area = island_area(r, c, &grid, &mut seen);
                    set_area(r, c, area, &mut areas);
                    result = result.max(area);
                }
            }
        }

        //println!("areas:{:?}", areas);
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 {
                    let mut n_res = 1;
                    if let Some(pr) = r.checked_sub(1) {
                        n_res += areas[pr][c];
                    }
                    if let Some(pc) = c.checked_sub(1) {
                        n_res += areas[r][pc];
                    }
                    if r < areas.len() - 1 {
                        n_res += areas[r + 1][c];
                    }
                    if c < areas[r].len() - 1 {
                        n_res += areas[r][c + 1];
                    }
                    result = result.max(n_res);
                }
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
    fn g_10_01_produces_3() {
        let grid = vv![[1, 0], [0, 1]];
        assert_eq!(Solution::largest_island(grid), 3);
        // Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
    }
    #[test]
    fn g_11_10_produces_4() {
        let grid = vv![[1, 1], [1, 0]];
        assert_eq!(Solution::largest_island(grid), 4);
        // Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
    }
    #[test]
    fn g_11_11_produces_4() {
        let grid = vv![[1, 1], [1, 1]];
        assert_eq!(Solution::largest_island(grid), 4);
        // Explanation: Can't change any 0 to 1, only one island with area = 4.
    }
    #[test]
    fn g_010_101_010_produces_5() {
        let grid = vv![[0, 1, 0], [1, 0, 1], [0, 1, 0]];
        assert_eq!(Solution::largest_island(grid), 5);
    }

    mod performance {
        use super::*;

        #[test]
        fn g_01x250_10x250_x250_produces_5() {
            let mut grid = vec![];
            for _ in 0..250 {
                grid.push(vec![0, 1].repeat(250));
                grid.push(vec![1, 0].repeat(250));
            }
            assert_eq!(Solution::largest_island(grid), 5);
        }
    }
}
