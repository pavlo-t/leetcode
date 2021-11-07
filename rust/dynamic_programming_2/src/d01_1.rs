#![allow(dead_code)]
/// 509. Fibonacci Number
/// =====================
///
/// The __Fibonacci numbers__, commonly denoted `F(n)` form a sequence, called the __Fibonacci sequence__,
/// such that each number is the sum of the two preceding ones, starting from `0` and `1`.
/// That is:
///
/// ```text
/// F(0) = 0, F(1) = 1
/// F(n) = F(n - 1) + F(n - 2), for n > 1.
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
    pub fn fib(n: i32) -> i32 {
        println!("fib({})", n);
        if n < 2 {
            n
        } else {
            let n = n as usize;
            let (mut penultimate, mut ultimate) = (0, 1);
            for _ in 2..=n {
                std::mem::swap(&mut penultimate, &mut ultimate);
                ultimate += penultimate;
            }
            ultimate
        }
    }
    pub fn fib_dp_vec_top_bottom(n: i32) -> i32 {
        println!("fib({})", n);
        let n = n as usize;
        let mut dp = vec![0; n + 2];
        dp[1] = 1;
        for n in 2..=n {
            dp[n] = dp[n - 1] + dp[n - 2];
        }
        dp[n]
    }
    pub fn fib_rec_with_memo(n: i32) -> i32 {
        println!("fib({})", n);
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if n < 2 {
                n as i32
            } else if memo[n] >= 0 {
                memo[n]
            } else {
                memo[n] = rec(n - 1, memo) + rec(n - 2, memo);
                memo[n]
            }
        }
        let n = n as usize;
        let mut memo = vec![-1; n + 1];
        rec(n, &mut memo)
    }
    pub fn fib_rec(n: i32) -> i32 {
        println!("fib({})", n);
        fn rec(n: i32) -> i32 {
            if n < 2 {
                n
            } else {
                rec(n - 1) + rec(n - 2)
            }
        }
        rec(n)
    }
    pub fn fib_rec_self(n: i32) -> i32 {
        println!("fib({})", n);
        match n {
            0 => 0,
            1 => 1,
            n => Self::fib(n - 1) + Self::fib(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_0() { assert_eq!(Solution::fib(0), 0); }
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::fib(1), 1); }
    // Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::fib(2), 1); }
    // Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::fib(3), 2); }
    // Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::fib(4), 3); }
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::fib(5), 5); }
    #[rustfmt::skip] #[test] fn n_6() { assert_eq!(Solution::fib(6), 8); }
    #[rustfmt::skip] #[test] fn n_7() { assert_eq!(Solution::fib(7), 13); }

    #[rustfmt::skip] #[test] fn n_30() { assert_eq!(Solution::fib(30), 832_040); }
    #[rustfmt::skip] #[test] fn n_46() { assert_eq!(Solution::fib(46), 1_836_311_903); }
}
