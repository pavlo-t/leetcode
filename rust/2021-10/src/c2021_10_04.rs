#![allow(dead_code)]
/// 463. Island Perimeter
/// =====================
///
/// You are given `row x col` `grid` representing a map
/// where `grid[i][j] = 1` represents land
/// and `grid[i][j] = 0` represents water.
///
/// Grid cells are connected __horizontally/vertically__ (not diagonally).
/// The `grid` is completely surrounded by water, and there is exactly one island
/// (i.e., one or more connected land cells).
///
/// The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island.
/// One cell is a square with side length 1.
/// The grid is rectangular, width and height don't exceed 100.
/// Determine the perimeter of the island.
///
/// __Constraints:__
///
/// - `1 <= grid.length, grid[i].length <= 100`
/// - `grid[i][j]` is `0` or `1`.
/// - There is exactly one island in `grid`.
///
/// https://leetcode.com/problems/island-perimeter/
struct Solution;
impl Solution {
    /// Approach 2: Better Counting
    /// https://leetcode.com/problems/island-perimeter/solution/
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    perimeter += 4;
                    perimeter -= r.checked_sub(1).map(|r| grid[r][c]).unwrap_or(0) * 2;
                    perimeter -= c.checked_sub(1).map(|c| grid[r][c]).unwrap_or(0) * 2;
                }
            }
        }
        perimeter
    }
    pub fn island_perimeter_my(grid: Vec<Vec<i32>>) -> i32 {
        //grid.iter().for_each(|r| println!("  {:?}", r));
        let mut perimeter = 0;
        let lr = grid.len() - 1;
        let lc = grid[0].len() - 1;
        for r in 0..=lr {
            for c in 0..=lc {
                if grid[r][c] == 1 {
                    if r == 0 || grid[r - 1][c] == 0 {
                        perimeter += 1;
                    }
                    if r == lr || grid[r + 1][c] == 0 {
                        perimeter += 1;
                    }
                    if c == 0 || grid[r][c - 1] == 0 {
                        perimeter += 1;
                    }
                    if c == lc || grid[r][c + 1] == 0 {
                        perimeter += 1;
                    }
                }
            }
        }
        perimeter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn g_0100_1110_0100_1100() {
        #[rustfmt::skip]
        let g = vv![
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 1, 0, 0],
            [1, 1, 0, 0]];
        assert_eq!(Solution::island_perimeter(g), 16);
    }
    #[test]
    fn g_1() {
        let g = vv![[1]];
        assert_eq!(Solution::island_perimeter(g), 4);
    }
    #[test]
    fn g_10() {
        let g = vv![[1, 0]];
        assert_eq!(Solution::island_perimeter(g), 4);
    }
    #[test]
    fn g_100x100_1() {
        let g = vec![vec![1; 100]; 100];
        assert_eq!(Solution::island_perimeter(g), 400);
    }
}
