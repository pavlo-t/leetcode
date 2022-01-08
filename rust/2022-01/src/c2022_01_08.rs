#![allow(dead_code)]
/// 1463. Cherry Pickup II
/// ======================
///
/// You are given a `rows x cols` matrix `grid` representing a field of cherries
/// where `grid[i][j]` represents the number of cherries that you can collect from the `(i, j)` cell.
///
/// You have two robots that can collect cherries for you:
///
/// - __Robot #1__ is located at the __top-left__ corner `(0, 0)`, and
/// - __Robot #2__ is located at the __top-right__ corner `(0, cols - 1)`.
///
/// Return _the maximum number of cherries collection using both robots by following the rules below_:
///
/// - From a cell `(i, j)`, robots can move to cell `(i + 1, j - 1)`, `(i + 1, j)`, or `(i + 1, j + 1)`.
/// - When any robot passes through a cell, It picks up all cherries, and the cell becomes an empty cell.
/// - When both robots stay in the same cell, only one takes the cherries.
/// - Both robots cannot move outside of the grid at any moment.
/// - Both robots should reach the bottom row in `grid`.
///
/// __Constraints:__
///
/// - `rows == grid.length`
/// - `cols == grid[i].length`
/// - `2 <= rows, cols <= 70`
/// - `0 <= grid[i][j] <= 100`
///
/// https://leetcode.com/problems/cherry-pickup-ii/
struct Solution;
impl Solution {
    pub fn cherry_pickup_rec_brute_force(grid: Vec<Vec<i32>>) -> i32 {
        fn rec(r: usize, a: usize, b: usize, g: &[Vec<i32>]) -> i32 {
            if r == g.len() {
                0
            } else {
                let cols = g[0].len();
                let curr = if a == b { g[r][a] } else { g[r][a] + g[r][b] };
                let next = (a.saturating_sub(1)..=(a + 1))
                    .flat_map(|a| (b.saturating_sub(1)..=(b + 1)).map(move |b| (a, b)))
                    .filter(|&(a, b)| a < cols && b < cols)
                    .map(|(a, b)| rec(r + 1, a, b, g))
                    .max()
                    .unwrap();
                curr + next
            }
        }

        let cols = grid[0].len();
        rec(0, 0, cols - 1, &grid)
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        fn rec(r: usize, a: usize, b: usize, g: &[Vec<i32>], memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if r == g.len() {
                0
            } else if memo[r][a][b] != -1 {
                memo[r][a][b]
            } else {
                memo[r][a][b] = {
                    let cols = g[0].len();
                    let curr = if a == b { g[r][a] } else { g[r][a] + g[r][b] };
                    let next = (a.saturating_sub(1)..=(a + 1))
                        .flat_map(|a| (b.saturating_sub(1)..=(b + 1)).map(move |b| (a, b)))
                        .filter(|&(a, b)| a < cols && b < cols)
                        .map(|(a, b)| rec(r + 1, a, b, g, memo))
                        .max()
                        .unwrap();
                    curr + next
                };
                memo[r][a][b]
            }
        }

        let (rows, cols) = (grid.len(), grid[0].len());
        let mut memo = vec![vec![vec![-1; cols]; cols]; rows];
        rec(0, 0, cols - 1, &grid, &mut memo)
    }

    pub fn cherry_pickup_dp_3d(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![0; cols + 2]; cols + 2]; rows + 1];
        for r in (0..rows).rev() {
            for a in 1..=cols {
                for b in 1..=cols {
                    let curr = grid[r][a - 1] + if a == b { 0 } else { grid[r][b - 1] };
                    let next = (a - 1..=a + 1)
                        .flat_map(|a| (b - 1..=b + 1).map(move |b| (a, b)))
                        .map(|(a, b)| dp[r + 1][a][b])
                        .max()
                        .unwrap();
                    dp[r][a][b] = curr + next;
                }
            }
        }

        dp[0][1][cols]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn g_311_251_155_211() {
        let g = vv![[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]];
        assert_eq!(Solution::cherry_pickup(g), 24);
        // Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
        // Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
        // Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
        // Total of cherries: 12 + 12 = 24.
    }
    #[test]
    fn g_1000001_2000030_2090000_0305400_1023006() {
        let g = vv![
            [1, 0, 0, 0, 0, 0, 1],
            [2, 0, 0, 0, 0, 3, 0],
            [2, 0, 9, 0, 0, 0, 0],
            [0, 3, 0, 5, 4, 0, 0],
            [1, 0, 2, 3, 0, 0, 6]
        ];
        assert_eq!(Solution::cherry_pickup(g), 28);
        // Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
        // Cherries taken by Robot #1, (1 + 9 + 5 + 2) = 17.
        // Cherries taken by Robot #2, (1 + 3 + 4 + 3) = 11.
        // Total of cherries: 17 + 11 = 28.
    }

    #[test]
    fn g_70x70x1() {
        let g = vec![vec![1; 70]; 70];
        assert_eq!(Solution::cherry_pickup(g), 140);
    }
}
