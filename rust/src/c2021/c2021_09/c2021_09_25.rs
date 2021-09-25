#![allow(dead_code)]
/// Shortest Path in a Grid with Obstacles Elimination
/// ==================================================
///
/// You are given an `m x n` integer matrix `grid` where each cell is either `0` (empty) or `1` (obstacle).
/// You can move up, down, left, or right from and to an empty cell in __one step__.
///
/// Return _the minimum number of __steps__ to walk from the upper left corner `(0, 0)`
/// to the lower right corner `(m - 1, n - 1)` given that you can eliminate __at most__ `k` obstacles_.
/// If it is not possible to find such walk return `-1`.
///
/// __Constraints:__
///
/// - `m == grid.length`
/// - `n == grid[i].length`
/// - `1 <= m, n <= 40`
/// - `1 <= k <= m * n`
/// - `grid[i][j] == 0 or 1`
/// - `grid[0][0] == grid[m - 1][n - 1] == 0`
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3987/
struct Solution;
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        grid.iter().for_each(|r| println!("  {:?}", r));
        println!("  {}", k);

        use std::collections::VecDeque;
        use std::iter;

        #[rustfmt::skip]
        fn can_visit(
            r: usize, c: usize, visited: &[Vec<i32>], grid: &[Vec<i32>], k: i32
        ) -> Option<(usize, usize, i32)> {
            if r < grid.len() && c < grid[0].len() && visited[r][c] < k {
                if k >= grid[r][c] {
                    Some((r, c, k - grid[r][c]))
                } else {
                    None
                }
            } else {
                None
            }
        }

        let target = (grid.len() - 1, grid[0].len() - 1);

        let mut q = VecDeque::new();
        q.push_back((0, 0, k, 0));

        let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];
        visited[0][0] = k;

        while let Some((r, c, k, mut rsf)) = q.pop_front() {
            if (r, c) == target {
                return rsf;
            }
            rsf += 1;
            iter::once((r + 1, c))
                .chain(iter::once((r, c + 1)))
                .chain(iter::once((r.wrapping_sub(1), c)))
                .chain(iter::once((r, c.wrapping_sub(1))))
                .for_each(|(r, c)| {
                    if let Some((r, c, k)) = can_visit(r, c, &visited, &grid, k) {
                        visited[r][c] = k;
                        q.push_back((r, c, k, rsf));
                    }
                });
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn g000_110_000_011_000_k1() {
        #[rustfmt::skip]
        let grid = vv![
            [0, 0, 0],
            [1, 1, 0],
            [0, 0, 0],
            [0, 1, 1],
            [0, 0, 0]];
        assert_eq!(Solution::shortest_path(grid, 1), 6);
        // Explanation:
        // The shortest path without eliminating any obstacle is 10.
        // The shortest path with one obstacle elimination at position (3,2) is 6.
        // Such path is (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
    }
    #[test]
    fn g011_111_100_k1() {
        #[rustfmt::skip]
        let grid = vv![
            [0, 1, 1],
            [1, 1, 1],
            [1, 0, 0]];
        assert_eq!(Solution::shortest_path(grid, 1), -1);
        // Explanation:
        // We need to eliminate at least two obstacles to find such a walk.
    }
    #[test]
    fn g0_k0() {
        assert_eq!(Solution::shortest_path(vv![[0]], 0), 0);
    }
    #[test]
    fn g00_k0() {
        assert_eq!(Solution::shortest_path(vv![[0, 0]], 0), 1);
    }
    #[test]
    fn g0_0_k0() {
        assert_eq!(Solution::shortest_path(vv![[0], [0]], 0), 1);
    }
    #[test]
    fn g00_00_k0() {
        assert_eq!(Solution::shortest_path(vv![[0, 0], [0, 0]], 0), 2);
    }
    #[test]
    fn g01_10_k0() {
        assert_eq!(Solution::shortest_path(vv![[0, 1], [1, 0]], 0), -1);
    }
    #[test]
    fn g01_10_k1() {
        assert_eq!(Solution::shortest_path(vv![[0, 1], [1, 0]], 1), 2);
    }
    #[test]
    fn g00_10_10_10_10_10_00_01_01_01_00_10_10_00_k4() {
        let grid = vv![
            [0, 0],
            [1, 0],
            [1, 0],
            [1, 0],
            [1, 0],
            [1, 0],
            [0, 0],
            [0, 1],
            [0, 1],
            [0, 1],
            [0, 0],
            [1, 0],
            [1, 0],
            [0, 0]
        ];
        assert_eq!(Solution::shortest_path(grid, 4), 14);
    }

    mod performance {
        use super::*;

        //#[ignore]
        #[test]
        fn g10x10_0s_k0() {
            let grid = vec![vec![0; 10]; 10];
            assert_eq!(Solution::shortest_path(grid, 0), 18);
        }
        //#[ignore]
        #[test]
        fn g20x20_0s_k0() {
            let grid = vec![vec![0; 20]; 20];
            assert_eq!(Solution::shortest_path(grid, 0), 38);
        }
        //#[ignore]
        #[test]
        fn g40x40_0s_k0() {
            let grid = vec![vec![0; 40]; 40];
            assert_eq!(Solution::shortest_path(grid, 0), 78);
        }
    }
}
