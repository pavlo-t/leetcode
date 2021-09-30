#![allow(dead_code)]

/// Fibonacci Number
/// ================
///
/// The __Fibonacci numbers__, commonly denoted `F(n)` form a sequence,
/// called the __Fibonacci sequence__, such that each number is the sum of the two preceding ones,
/// starting from 0 and 1. That is,
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
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3709/
struct Solution;

const SQRT5: f64 = 2.23606797749979;
const GOLDEN_RATIO: f64 = 1.618033988749895;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        ((GOLDEN_RATIO.powf(n as f64) - (-1f64 / GOLDEN_RATIO).powf(n as f64)) / SQRT5) as i32
    }

    pub fn fib_golden_round(n: i32) -> i32 {
        (GOLDEN_RATIO.powi(n) / SQRT5).round() as i32
    }
    pub fn fib_fold_no_overflow_on_46(n: i32) -> i32 {
        if n == 0 {
            0
        } else {
            (1..n).fold((0, 1), |(pp, p), _| (p, pp + p)).1
        }
    }
    pub fn fib_fold(n: i32) -> i32 {
        (0..n).fold((0, 1), |(pp, p), _| (p, pp + p)).0
    }
    pub fn fib_rec(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            Self::fib(n - 1) + Self::fib(n - 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0_produces_0() {
        assert_eq!(Solution::fib(0), 0);
    }
    #[test]
    fn n1_produces_1() {
        assert_eq!(Solution::fib(1), 1);
    }
    #[test]
    fn example1_n2_produces_1() {
        assert_eq!(Solution::fib(2), 1);
        // Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
    }
    #[test]
    fn example2_n3_produces_2() {
        assert_eq!(Solution::fib(3), 2);
        // Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
    }
    #[test]
    fn example3_n4_produces_3() {
        assert_eq!(Solution::fib(4), 3);
        // Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
    }

    #[test]
    fn n30_produces_832_040() {
        assert_eq!(Solution::fib(30), 832_040);
    }
    #[test]
    fn n45_produces_1_134_903_170() {
        assert_eq!(Solution::fib(45), 1134903170);
    }
    #[test]
    fn n46_produces_1_836_311_903() {
        assert_eq!(Solution::fib(46), 1836311903);
    }
}
