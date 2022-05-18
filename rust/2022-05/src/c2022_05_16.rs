#![allow(dead_code)]
/// \#1091. Shortest Path in Binary Matrix
/// ====================================
///
/// Given an `n x n` binary matrix `grid`, return _the length of the shortest __clear path__ in the matrix_.
/// If there is no clear path, return `-1`.
///
/// A __clear path__ in a binary matrix is a path from the __top-left__ cell (i.e., `(0, 0)`)
/// to the __bottom-right__ cell (i.e., `(n - 1, n - 1)`) such that:
///
/// - All the visited cells of the path are `0`.
/// - All the adjacent cells of the path are __8-directionally__ connected
///   (i.e., they are different and they share an edge or a corner).
///
/// The __length of a clear path__ is the number of visited cells of this path.
///
/// ### Constraints:
///
/// - `n == grid.length`
/// - `n == grid[i].length`
/// - `1 <= n <= 100`
/// - `grid[i][j] is 0 or 1`
///
/// https://leetcode.com/problems/shortest-path-in-binary-matrix/
struct Solution;
impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            -1
        } else {
            use std::collections::VecDeque;

            let n = grid.len();
            let t = n - 1;

            let mut nexts = VecDeque::new();
            nexts.push_back((0, 0, 1));
            grid[0][0] = 2;

            while let Some((r, c, path)) = nexts.pop_front() {
                if r == t && c == t {
                    return path;
                }
                let next_path = path + 1;
                for next_r in r.saturating_sub(1)..(r + 2).min(n) {
                    for next_c in c.saturating_sub(1)..(c + 2).min(n) {
                        if (next_r == r && next_c == c) || grid[next_r][next_c] != 0 {
                            continue;
                        }
                        nexts.push_back((next_r, next_c, next_path));
                        grid[next_r][next_c] = 2;
                    }
                }
            }

            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] } }

    #[rustfmt::skip] #[test] fn g_0() { assert_eq!(Solution::shortest_path_binary_matrix(vv![[0]]), 1); }
    #[rustfmt::skip] #[test] fn g_1() { assert_eq!(Solution::shortest_path_binary_matrix(vv![[1]]), -1); }

    #[test]
    fn g_01_10() {
        let g = vv![[0, 1], [1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(g), 2);
    }
    #[test]
    fn g_000_110_110() {
        let g = vv![[0, 0, 0], [1, 1, 0], [1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(g), 4);
    }
    #[test]
    fn g_100_110_110() {
        let g = vv![[1, 0, 0], [1, 1, 0], [1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(g), -1);
    }

    #[test]
    fn g_100x100x0() {
        let g = vec![vec![0; 100]; 100];
        assert_eq!(Solution::shortest_path_binary_matrix(g), 100);
    }
    #[test]
    fn g_100x100x0_bottom_right_1() {
        let mut g = vec![vec![0; 100]; 100];
        g[99][99] = 1;
        assert_eq!(Solution::shortest_path_binary_matrix(g), -1);
    }
}
