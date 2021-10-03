#![allow(dead_code)]
/// 70. Climbing Stairs
/// ===================
///
/// You are climbing a staircase. It takes `n` steps to reach the top.
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
        if n < 4 {
            n
        } else {
            let mut p = 3;
            let mut pp = 2;
            for _ in 4..=n {
                std::mem::swap(&mut p, &mut pp);
                p += pp;
            }
            p
        }
    }
    pub fn climb_stairs_dp(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![-1; (n + 1).max(4)];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;
        for i in 4..=n {
            dp[i] = dp[i - 1] + dp[i - 2]
        }
        dp[n]
    }
    pub fn climb_stairs_rec_with_memo(n: i32) -> i32 {
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if memo[n] > 0 {
                memo[n]
            } else {
                let r1 = rec(n - 1, memo);
                let r2 = rec(n - 2, memo);
                memo[n] = r1 + r2;
                memo[n]
            }
        }
        let n = n as usize;
        let mut memo = vec![-1; (n + 1).max(4)];
        memo[1] = 1;
        memo[2] = 2;
        memo[3] = 3;
        rec(n, &mut memo)
    }
    pub fn climb_stairs_rec(n: i32) -> i32 {
        #[rustfmt::skip]
        fn rec(n: i32) -> i32 {
            if n < 4 { n } else { rec(n - 1) + rec(n - 2) }
        }
        rec(n)
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
    // 1111 112 121 211 22
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::climb_stairs(4), 5); }
    // 11111 1112 1121 1211 2111 122 212 221
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::climb_stairs(5), 8); }
    // 111111 11112 11121 11211 12111 21111 1122 1212 2112 1221 2121 2211 222
    #[rustfmt::skip] #[test] fn n_6() { assert_eq!(Solution::climb_stairs(6), 13); }
    #[rustfmt::skip] #[test] fn n_45() { assert_eq!(Solution::climb_stairs(45), 1_836_311_903); }
}
