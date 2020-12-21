#![allow(dead_code)]

struct Solution;

//noinspection DuplicatedCode
impl Solution {
    // bottom up dp
    pub fn cherry_pickup_bottom_up_dp(grid: Vec<Vec<i32>>) -> i32 {
    // pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![vec![0; m + 2]; m + 2]; n + 1];

        for i in (0..n).rev() {
            for r1 in 1..m {
                for r2 in r1 + 1..=m {
                    let mut next_max = 0;
                    for nr1 in r1 - 1..=r1 + 1 {
                        for nr2 in r2 - 1..=r2 + 1 {
                            if nr1 < nr2 {
                                next_max = next_max.max(dp[i + 1][nr1][nr2])
                            }
                        }
                    }
                    dp[i][r1][r2] = grid[i][r1 - 1] + grid[i][r2 - 1] + next_max;
                }
            }
        }

        dp[0][1][m]
    }

    // top down dp / memoization
    // pub fn cherry_pickup_top_down_memoization(grid: Vec<Vec<i32>>) -> i32 {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &Vec<Vec<i32>>, i: usize, r1: usize, r2: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if i == grid.len() {
                0
            } else if dp[i][r1][r2] != -1 {
                dp[i][r1][r2]
            } else {
                let mut max_next = 0;
                for nr1 in if r1 == 0 { r1 } else { r1 - 1 }..=r1 + 1 {
                    for nr2 in r2 - 1..=r2 + 1 {
                        if nr1 < nr2 && nr2 < grid[i].len() {
                            max_next = max_next.max(dfs(grid, i + 1, nr1, nr2, dp));
                        }
                    }
                }
                dp[i][r1][r2] = grid[i][r1] + grid[i][r2] + max_next;
                dp[i][r1][r2]
            }
        }

        let mut dp = vec![vec![vec![-1; grid[0].len()]; grid[0].len()]; grid.len()];
        dfs(&grid, 0, 0, grid[0].len() - 1, &mut dp)
    }

    pub fn cherry_pickup_bruteforce(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &Vec<Vec<i32>>, i: usize, r1: usize, r2: usize) -> i32 {
            if i == grid.len() {
                0
            } else {
                let mut max_next = 0;
                for nr1 in if r1 == 0 { r1 } else { r1 - 1 }..=r1 + 1 {
                    for nr2 in r2 - 1..=r2 + 1 {
                        if nr1 < nr2 && nr2 < grid[i].len() {
                            max_next = max_next.max(dfs(grid, i + 1, nr1, nr2));
                        }
                    }
                }
                grid[i][r1] + grid[i][r2] + max_next
            }
        }

        dfs(&grid, 0, 0, grid[0].len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_g311_251_155_211_is_24() {
        let grid = vec![
            vec![3, 1, 1],
            vec![2, 5, 1],
            vec![1, 5, 5],
            vec![2, 1, 1]];
        assert_eq!(Solution::cherry_pickup(grid), 24);
        //Explanation:
        // Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
        // Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
        // Total of cherries: 12 + 12 = 24.
    }

    #[test]
    fn example2_g1000001_2000030_0305400_1023006_is_28() {
        let grid = vec![
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![2, 0, 0, 0, 0, 3, 0],
            vec![2, 0, 9, 0, 0, 0, 0],
            vec![0, 3, 0, 5, 4, 0, 0],
            vec![1, 0, 2, 3, 0, 0, 6]];
        assert_eq!(Solution::cherry_pickup(grid), 28);
        //Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
        // Cherries taken by Robot #1, (1 + 0 + 9 + 5 + 2) = 17.
        // Cherries taken by Robot #2, (1 + 3 + 0 + 4 + 3) = 11.
        // Total of cherries: 17 + 11 = 28.
    }

    #[test]
    fn example3_g1003_0003_0033_9033_is_22() {
        let grid = vec![
            vec![1, 0, 0, 3],
            vec![0, 0, 0, 3],
            vec![0, 0, 3, 3],
            vec![9, 0, 3, 3]];
        assert_eq!(Solution::cherry_pickup(grid), 22);
    }

    #[test]
    fn example4_g11_11_is_4() {
        let grid = vec![
            vec![1, 1],
            vec![1, 1]];
        assert_eq!(Solution::cherry_pickup(grid), 4);
    }

    #[test]
    fn g15_15_is_12() {
        let grid = vec![
            vec![1, 5],
            vec![1, 5]];
        assert_eq!(Solution::cherry_pickup(grid), 12);
    }

    #[test]
    fn g51_51_is_12() {
        let grid = vec![
            vec![5, 1],
            vec![5, 1]];
        assert_eq!(Solution::cherry_pickup(grid), 12);
    }

    #[test]
    fn g111_111_111_is_6() {
        let grid = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1]];
        assert_eq!(Solution::cherry_pickup(grid), 6);
    }

    #[test]
    fn g123_456_789_is_6() {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]];
        assert_eq!(Solution::cherry_pickup(grid), 32);
    }

    #[test]
    fn g70x70_filled_with_0_is_0() {
        let grid = vec![vec![0; 70]; 70];
        assert_eq!(Solution::cherry_pickup(grid), 0);
    }

    #[test]
    fn g70x70_filled_with_1_is_1() {
        let grid = vec![vec![1; 70]; 70];
        assert_eq!(Solution::cherry_pickup(grid), 140);
    }
}
