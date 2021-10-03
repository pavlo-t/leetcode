#![allow(dead_code)]
/// 1137. N-th Tribonacci Number
/// ============================
///
/// The Tribonacci sequence Tn is defined as follows:
///
/// `T0 = 0`, `T1 = 1`, `T2 = 1`, and `Tn+3 = Tn + Tn+1 + Tn+2` for `n >= 0`.
///
/// Given `n`, return the value of Tn.
///
/// __Constraints:__
///
/// - `0 <= n <= 37`
/// - The answer is guaranteed to fit within a 32-bit integer, ie. `answer <= 2^31 - 1`.
///
/// https://leetcode.com/problems/n-th-tribonacci-number/
struct Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        println!("tribonacci({})", n);
        let mut p = [0, 1, 1];
        for _ in 3..=n {
            p.swap(0, 1);
            p.swap(1, 2);
            p[2] = p.iter().sum();
        }
        p[(n as usize).min(2)]
    }

    pub fn tribonacci_dp(n: i32) -> i32 {
        println!("tribonacci({})", n);
        let n = n as usize;
        let mut dp = vec![0; (n + 1).max(3)];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n0() { assert_eq!(Solution::tribonacci(0), 0); }
    #[rustfmt::skip] #[test] fn n1() { assert_eq!(Solution::tribonacci(1), 1); }
    #[rustfmt::skip] #[test] fn n2() { assert_eq!(Solution::tribonacci(2), 1); }
    #[rustfmt::skip] #[test] fn n3() { assert_eq!(Solution::tribonacci(3), 2); }
    #[test]
    fn n4() {
        assert_eq!(Solution::tribonacci(4), 4);
        // Explanation:
        // T_3 = 0 + 1 + 1 = 2
        // T_4 = 1 + 1 + 2 = 4
    }
    #[test]
    fn n25() {
        assert_eq!(Solution::tribonacci(25), 1_389_537);
    }
}
