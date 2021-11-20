#![allow(dead_code)]
/// 62. Unique Paths
/// ================
///
/// https://leetcode.com/problems/unique-paths/
struct Solution;
impl Solution {
    pub fn unique_paths_rec(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        fn rec(m: usize, n: usize) -> i32 {
            if m == 1 || n == 1 {
                1
            } else {
                rec(m - 1, n) + rec(m, n - 1)
            }
        }
        let (m, n) = (m as usize, n as usize);
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
    pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n]; m];
        for r in 1..m {
            for c in 1..n {
                dp[r][c] = dp[r - 1][c] + dp[r][c - 1];
            }
        }
        dp[m - 1][n - 1]
    }
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        println!("unique_paths({}, {})", m, n);
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n];
        for _ in 1..m {
            for c in 1..n {
                dp[c] += dp[c - 1];
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m1n1() {
        assert_eq!(Solution::unique_paths(1, 1), 1);
    }
    #[test]
    fn m2n1() {
        assert_eq!(Solution::unique_paths(2, 1), 1);
    }
    #[test]
    fn m1n2() {
        assert_eq!(Solution::unique_paths(1, 2), 1);
    }
    #[test]
    fn m2n2() {
        assert_eq!(Solution::unique_paths(2, 2), 2);
    }
    #[test]
    fn m3n7() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
    #[test]
    fn m3n2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        // Explanation:
        // From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
        // 1. Right -> Down -> Down
        // 2. Down -> Down -> Right
        // 3. Down -> Right -> Down
    }
    #[test]
    fn m7n3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }
    #[test]
    fn m3n3() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }

    #[test]
    fn m18n17() {
        assert_eq!(Solution::unique_paths(18, 17), 1166803110);
    }
    #[test]
    fn m17n18() {
        assert_eq!(Solution::unique_paths(17, 18), 1166803110);
    }
}
