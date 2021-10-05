#![allow(dead_code)]
/// 70. Climbing Stairs
/// ===================
///
/// You are climbing a staircase.
/// It takes `n` steps to reach the top.
///
/// Each time you can either climb `1` or `2` steps.
/// In how many distinct ways can you climb to the top?
///
/// __Constraints:__
///
/// - `1 <= n <= 45`
///
/// https://leetcode.com/problems/climbing-stairs/
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        for _ in 2..=n {
            std::mem::swap(&mut a, &mut b);
            a += b;
        }
        a
    }
    pub fn climb_stairs_dp_constant_space(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1);
        for _ in 2..=n {
            let tmp = a;
            a += b;
            b = tmp;
        }
        a
    }
    pub fn climb_stairs_dp(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n {
            dp[i] = dp[i - 2] + dp[i - 1];
        }
        dp[n]
    }
    pub fn climb_stairs_dfs_with_memo(n: i32) -> i32 {
        fn dfs(n: usize, memo: &mut Vec<i32>) -> i32 {
            if memo[n] >= 0 {
                memo[n]
            } else {
                memo[n] = dfs(n - 1, memo) + dfs(n - 2, memo);
                memo[n]
            }
        }
        let n = n as usize;
        let mut memo = vec![-1; n + 1];
        memo[0] = 1;
        memo[1] = 1;
        dfs(n, &mut memo)
    }
    pub fn climb_stairs_dfs(n: i32) -> i32 {
        fn dfs(n: i32) -> i32 {
            match n {
                1 => 1,
                2 => 2,
                n => dfs(n - 1) + dfs(n - 2),
            }
        }
        dfs(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::climb_stairs(1), 1); }
    #[test]
    fn n_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
        // Explanation: There are two ways to climb to the top.
        // 1. 1 step + 1 step
        // 2. 2 steps
    }
    #[test]
    fn n_3() {
        assert_eq!(Solution::climb_stairs(3), 3);
        // Explanation: There are three ways to climb to the top.
        // 1. 1 step + 1 step + 1 step
        // 2. 1 step + 2 steps
        // 3. 2 steps + 1 step
    }
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::climb_stairs(4), 5); }
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::climb_stairs(5), 8); }

    #[rustfmt::skip] #[test] fn n_45() { assert_eq!(Solution::climb_stairs(45), 1_836_311_903); }
}
