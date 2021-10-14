#![allow(dead_code)]
/// 279. Perfect Squares
/// ====================
///
/// Given an integer `n`, return _the least number of perfect square numbers that sum to `n`_.
///
/// A __perfect square__ is an integer that is the square of an integer;
/// in other words, it is the product of some integer with itself.
/// For example, `1`, `4`, `9`, and `16` are perfect squares while `3` and `11` are not.
///
/// __Constraints:__
///
/// - `1 <= n <= 10_000`
///
/// https://leetcode.com/problems/perfect-squares/
struct Solution;
impl Solution {
    pub fn num_squares_dp(n: i32) -> i32 {
        let ns = n as usize;
        let mut dp = vec![vec![i32::MAX; 101]; ns + 1];
        for n in 0..1 {
            for r in 1..=100 {
                dp[n][r] = n as i32;
            }
        }
        for n in 1..=ns {
            dp[n][1] = n as i32;
            for r in 2..=100 {
                let s = r * r;
                dp[n][r] = if n < s {
                    dp[n][r - 1]
                } else {
                    dp[n][r - 1].min(1 + dp[n - s][r])
                }
            }
        }
        dp[ns].to_owned().into_iter().min().unwrap()
    }
    pub fn num_squares(n: i32) -> i32 {
        println!(" NS({})", n);
        fn rec(n: i32, r: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
            if r <= 0 {
                i32::MAX
            } else if n < 4 {
                n
            } else if memo[n as usize][r as usize] >= 0 {
                memo[n as usize][r as usize]
            } else {
                let s = r * r;
                memo[n as usize][r as usize] = if n < s {
                    rec(n, r - 1, memo)
                } else {
                    rec(n, r - 1, memo).min(1 + rec(n - s, r, memo))
                };
                memo[n as usize][r as usize]
            }
        }
        rec(n, 100, &mut vec![vec![-1; 101]; n as usize + 1])
    }
    pub fn num_squares_rec(n: i32) -> i32 {
        println!(" NS({})", n);
        fn rec(n: i32, r: i32) -> i32 {
            if n < 4 || r == 0 {
                n
            } else {
                let s = r * r;
                if n < s {
                    rec(n, r - 1)
                } else {
                    rec(n, r - 1).min(1 + rec(n - s, r))
                }
            }
        }
        rec(n, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_12() {
        assert_eq!(Solution::num_squares(12), 3);
        // Explanation: 12 = 4 + 4 + 4.
    }
    #[test]
    fn n_13() {
        assert_eq!(Solution::num_squares(13), 2);
        // Explanation: 13 = 4 + 9.
    }
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::num_squares(1), 1); }
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::num_squares(2), 2); }
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::num_squares(3), 3); }
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::num_squares(4), 1); }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2021_10_14::tests::n_10000' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=67108864` to env:
    ///
    /// ```sh
    /// RUST_MIN_STACK=67108864 cargo test --lib c2021_10_14
    /// ```
    #[rustfmt::skip] #[test] fn n_10000() { assert_eq!(Solution::num_squares(10_000), 1); }
}
