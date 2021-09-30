#![allow(dead_code)]
/// Unique Paths II
/// ===============
///
/// A robot is located at the top-left corner of a `m x n` grid.
///
/// The robot can only move either down or right at any point in time.
/// The robot is trying to reach the bottom-right corner of the grid.
///
/// Now consider if some obstacles are added to the grids. How many unique paths would there be?
///
/// An obstacle and space is marked as `1` and `0` respectively in the grid.
///
/// __Constraints:__
///
/// - `1 <= obstacleGrid.length, obstacleGrid[i].length <= 100`
/// - `obstacleGrid[i][j]` is `0` or `1`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3723/
struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 || obstacle_grid.last().unwrap().last().unwrap() == &1 {
            0
        } else {
            fn dfs(r: usize, c: usize, og: &[Vec<i32>], cache: &mut [Vec<i32>]) -> i32 {
                let rs = og.len();
                let cs = og[0].len();
                if r == rs - 1 && c == cs - 1 {
                    1
                } else if cache[r][c] >= 0 {
                    cache[r][c]
                } else {
                    let mut ways = 0;
                    if r < rs - 1 && og[r + 1][c] == 0 {
                        ways += dfs(r + 1, c, og, cache);
                    }
                    if c < cs - 1 && og[r][c + 1] == 0 {
                        ways += dfs(r, c + 1, og, cache)
                    }
                    cache[r][c] = ways;
                    ways
                }
            }
            let rs = obstacle_grid.len();
            let cs = obstacle_grid[0].len();
            dfs(0, 0, &obstacle_grid, &mut vec![vec![-1; cs]; rs])
        }
    }

    pub fn unique_paths_with_obstacles_brute_force(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 || obstacle_grid.last().unwrap().last().unwrap() == &1 {
            0
        } else {
            fn dfs(r: usize, c: usize, og: &[Vec<i32>]) -> i32 {
                let rs = og.len();
                let cs = og[0].len();
                if r == rs - 1 && c == cs - 1 {
                    1
                } else {
                    let mut ways = 0;
                    if r < rs - 1 && og[r + 1][c] == 0 {
                        ways += dfs(r + 1, c, og);
                    }
                    if c < cs - 1 && og[r][c + 1] == 0 {
                        ways += dfs(r, c + 1, og)
                    }
                    ways
                }
            }
            dfs(0, 0, &obstacle_grid)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_og000_010_000_produces_2() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 2);
        // Explanation: There is one obstacle in the middle of the 3x3 grid above.
        // There are two ways to reach the bottom-right corner:
        // 1. Right -> Right -> Down -> Down
        // 2. Down -> Down -> Right -> Right
    }
    #[test]
    fn example2_og01_00_produces_1() {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 1);
    }

    #[test]
    fn og1_produces_0() {
        let obstacle_grid = vec![vec![1]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }
    #[test]
    fn og0_produces_1() {
        let obstacle_grid = vec![vec![0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 1);
    }
    #[test]
    fn og010_produces_0() {
        let obstacle_grid = vec![vec![0, 1, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }
    #[test]
    fn og01_produces_0() {
        let obstacle_grid = vec![vec![0, 1]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }
    #[test]
    fn og00_01_produces_0() {
        let obstacle_grid = vec![vec![0, 0], vec![0, 1]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }
    #[test]
    fn og10_produces_0() {
        let obstacle_grid = vec![vec![1, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }
    #[test]
    fn og10_00_produces_0() {
        let obstacle_grid = vec![vec![1, 0], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }

    #[test]
    fn og00_00_produces_2() {
        let obstacle_grid = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 2);
    }
    #[test]
    fn og000_000_000_produces_6() {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 6);
    }

    #[test]
    #[should_panic]
    fn og100x100_zeroes_produces_overflow() {
        let obstacle_grid = vec![vec![0; 100]; 100];
        Solution::unique_paths_with_obstacles(obstacle_grid);
    }
    #[test]
    fn og15x16_zeroes_produces_1_391_975_640() {
        let obstacle_grid = vec![vec![0; 15]; 21];
        let e = 1_391_975_640;
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), e);
    }
}
