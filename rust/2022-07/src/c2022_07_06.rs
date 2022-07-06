#![allow(dead_code)]
//! \#509. Fibonacci Number
//! =======================
//!
//! The __Fibonacci numbers__, commonly denoted `F(n)` form a sequence, called the __Fibonacci sequence__,
//! such that each number is the sum of the two preceding ones, starting from `0` and `1`.
//! That is,
//!
//! ```text
//! F(0) = 0, F(1) = 1
//! F(n) = F(n - 1) + F(n - 2), for n > 1.
//! ```
//!
//! Given `n`, calculate `F(n)`.
//!
//! __Constraints:__
//!
//! - `0 <= n <= 30`
//!
//! <https://leetcode.com/problems/fibonacci-number>

pub struct Solution;
impl Solution {
    pub fn fib_rec(n: i32) -> i32 {
        if n < 2 {
            n
        } else {
            Self::fib(n - 1) + Self::fib(n - 2)
        }
    }

    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            n
        } else {
            let (mut prev, mut curr) = (0, 1);
            for _ in 1..n {
                std::mem::swap(&mut curr, &mut prev);
                curr += prev;
            }
            curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_0() { assert_eq!(Solution::fib(0), 0); }
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::fib(1), 1); }

    #[test]
    fn n_2() {
        assert_eq!(Solution::fib(2), 1);
        // Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
    }
    #[test]
    fn n_3() {
        assert_eq!(Solution::fib(3), 2);
        // Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
    }
    #[test]
    fn n_4() {
        assert_eq!(Solution::fib(4), 3);
        // Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
    }
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::fib(5), 5); }

    #[rustfmt::skip] #[test] fn n_30() { assert_eq!(Solution::fib(30), 832_040); }
}
