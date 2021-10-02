#![allow(dead_code)]
/// 509. Fibonacci Number
/// =====================
///
/// The __Fibonacci numbers__,
/// commonly denoted `F(n)` form a sequence,
/// called the __Fibonacci sequence__,
/// such that each number is the sum of the two preceding ones,
/// starting from `0` and `1`.
/// That is,
///
/// ```text
/// F(0) = 0, F(1) = 1
/// F(n) = F(n - 1) + F(n - 2), for n > 1
/// ```
///
/// Given `n`, calculate `F(n)`.
///
/// __Constraints:__
///
/// - `0 <= n <= 30`
///
/// https://leetcode.com/problems/fibonacci-number/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/fibonacci-number/solution/
    pub fn fib(n: i32) -> i32 {
        println!("fib({})", n);
        let sqrt5 = 5.0f64.sqrt();
        let golden_ratio = (1.0 + sqrt5) / 2.0;
        (golden_ratio.powi(n) / sqrt5).round() as i32
    }
    pub fn fib_constant_space_dp(n: i32) -> i32 {
        println!("fib({})", n);
        if n <= 1 {
            n
        } else {
            let mut p2 = 0;
            let mut p1 = 1;
            for _ in 2..=n {
                p1 = p1 + p2;
                p2 = p1 - p2;
            }
            p1
        }
    }
    pub fn fib_bottom_up_tabulation_dp(n: i32) -> i32 {
        println!("fib({})", n);
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            let n = n as usize;
            let mut dp = vec![0; n + 1];
            dp[1] = 1;
            for i in 2..=n {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            dp[n]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_0() {
        assert_eq!(Solution::fib(0), 0);
    }
    #[test]
    fn fib_1() {
        assert_eq!(Solution::fib(1), 1);
    }
    #[test]
    fn fib_2() {
        assert_eq!(Solution::fib(2), 1);
        // Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
    }
    #[test]
    fn fib_3() {
        assert_eq!(Solution::fib(3), 2);
        // Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
    }
    #[test]
    fn fib_4() {
        assert_eq!(Solution::fib(4), 3);
        // Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
    }
    #[test]
    fn fib_30() {
        assert_eq!(Solution::fib(30), 832_040);
    }
}
