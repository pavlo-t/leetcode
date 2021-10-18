#![allow(dead_code)]
/// 62. Unique Paths
/// ================
///
/// A robot is located at the top-left corner of a `m x n` grid.
///
/// The robot can only move either down or right at any point in time.
/// The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
///
/// How many possible unique paths are there?
///
/// __Constraints:__
///
/// - `1 <= m, n <= 100`
/// - It's guaranteed that the answer will be less than or equal to `2_000_000_000`.
///
/// https://leetcode.com/problems/unique-paths/
struct Solution;
impl Solution {
    /// 23:00-05
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![0; n + 1];
        dp[n - 1] = 1;
        for _ in (0..m).rev() {
            for c in (0..n).rev() {
                dp[c] += dp[c + 1];
            }
        }
        dp[0]
    }
    /// 22:54-23:00
    pub fn unique_paths_dp_vec_vec(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[m][n - 1] = 1;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                dp[r][c] = dp[r + 1][c] + dp[r][c + 1];
            }
        }
        dp[0][0]
    }
    /// 22:52-22:54
    pub fn unique_paths_rec_with_memo(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        fn rec(m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if m == 1 || n == 1 {
                1
            } else if memo[m][n] > 0 {
                memo[m][n]
            } else {
                memo[m][n] = rec(m - 1, n, memo) + rec(m, n - 1, memo);
                memo[m][n]
            }
        }
        let (m, n) = (m as usize, n as usize);
        rec(m, n, &mut vec![vec![0; n + 1]; m + 1])
    }
    /// 22:45-22:51
    pub fn unique_paths_rec(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        fn rec(m: i32, n: i32) -> i32 {
            if m == 1 || n == 1 {
                1
            } else {
                rec(m - 1, n) + rec(m, n - 1)
            }
        }
        rec(m, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn m3n7() { assert_eq!(Solution::unique_paths(3, 7), 28); }
    /// Explanation:
    /// From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
    /// 1. Right -> Down -> Down
    /// 2. Down -> Down -> Right
    /// 3. Down -> Right -> Down
    #[rustfmt::skip] #[test] fn m3n2() { assert_eq!(Solution::unique_paths(3, 2), 3); }
    #[rustfmt::skip] #[test] fn m7n3() { assert_eq!(Solution::unique_paths(7, 3), 28); }
    #[rustfmt::skip] #[test] fn m3n3() { assert_eq!(Solution::unique_paths(3, 3), 6); }
    #[rustfmt::skip] #[test] fn m100n1() { assert_eq!(Solution::unique_paths(100, 1), 1); }
    #[rustfmt::skip] #[test] fn m1n100() { assert_eq!(Solution::unique_paths(1, 100), 1); }

    #[rustfmt::skip] #[test] fn m100n7() { assert_eq!(Solution::unique_paths(100, 7), 1_609_344_100); }
    #[rustfmt::skip] #[test] fn m7n100() { assert_eq!(Solution::unique_paths(7, 100), 1_609_344_100); }
}
