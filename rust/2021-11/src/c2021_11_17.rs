#![allow(dead_code)]
/// 62. Unique Paths
/// ================
///
/// A robot is located at the top-left corner of a `m x n` grid.
///
/// The robot can only move either down or right at any point in time.
/// The robot is trying to reach the bottom-right corner of the grid.
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
    pub fn unique_paths_rec_with_memo(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        fn rec(m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[m][n] != -1 {
                memo[m][n]
            } else {
                memo[m][n] = if m == 1 || n == 1 {
                    1
                } else {
                    rec(m - 1, n, memo) + rec(m, n - 1, memo)
                };
                memo[m][n]
            }
        }
        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![-1; n + 1]; m + 1];
        rec(m, n, &mut memo)
    }
    pub fn unique_paths_dp_vec_vec(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n + 1]; m + 1];
        for i in 2..=m {
            for j in 2..=n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m][n]
    }
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];
        for _ in 1..m {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn m1_n1() { assert_eq!(Solution::unique_paths(1, 1), 1); }
    #[rustfmt::skip] #[test] fn m2_n1() { assert_eq!(Solution::unique_paths(2, 1), 1); }
    #[rustfmt::skip] #[test] fn m1_n2() { assert_eq!(Solution::unique_paths(1, 2), 1); }
    #[rustfmt::skip] #[test] fn m2_n2() { assert_eq!(Solution::unique_paths(2, 2), 2); }
    #[rustfmt::skip] #[test] fn m3_n7() { assert_eq!(Solution::unique_paths(3, 7), 28); }
    #[test]
    fn m3_n2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        // Explanation:
        // From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
        // 1. Right -> Down -> Down
        // 2. Down -> Down -> Right
        // 3. Down -> Right -> Down
    }
    #[rustfmt::skip] #[test] fn m7_n3() { assert_eq!(Solution::unique_paths(7, 3), 28); }
    #[rustfmt::skip] #[test] fn m3_n3() { assert_eq!(Solution::unique_paths(3, 3), 6); }
    #[rustfmt::skip] #[test] fn m17_n18() { assert_eq!(Solution::unique_paths(17, 18), 1_166_803_110); }
}
