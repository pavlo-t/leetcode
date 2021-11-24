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
    /// 23:16-23:23
    pub fn num_squares_rec(n: i32) -> i32 {
        println!("num_squares({})", n);
        fn rec(i: usize, n: i32, s: &[i32]) -> i32 {
            if n == 0 {
                0
            } else if n < 0 || i == s.len() {
                i32::MAX
            } else {
                rec(i, n - s[i], s).saturating_add(1).min(rec(i + 1, n, s))
            }
        }

        let max = (n as f64).sqrt() as i32;
        let squares = (1..=max).map(|i| i * i).collect::<Vec<_>>();
        rec(0, n, &squares)
    }
    /// 23:24-23:27
    pub fn num_squares_rec_with_memo(n: i32) -> i32 {
        println!("num_squares({})", n);
        fn rec(i: usize, n: i32, s: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if n == 0 {
                0
            } else if n < 0 || i == s.len() {
                i32::MAX
            } else if memo[i][n as usize] != -1 {
                memo[i][n as usize]
            } else {
                memo[i][n as usize] =
                    rec(i, n - s[i], s, memo)
                        .saturating_add(1)
                        .min(rec(i + 1, n, s, memo));
                memo[i][n as usize]
            }
        }

        let max_root = (n as f64).sqrt() as i32;
        let squares = (1..=max_root).map(|i| i * i).collect::<Vec<_>>();
        let mut memo = vec![vec![-1; n as usize + 1]; squares.len()];
        rec(0, n, &squares, &mut memo)
    }
    /// 23:27-23:41
    pub fn num_squares_dp_vec_vec(n: i32) -> i32 {
        println!("num_squares({})", n);
        let n = n as usize;
        let max_root = (n as f64).sqrt() as usize;
        let squares = (1..=max_root).map(|i| i * i).collect::<Vec<_>>();
        let sl = squares.len();
        let mut dp = vec![vec![i32::MAX; n + 1]; sl + 1];
        for s in (0..sl).rev() {
            dp[s][0] = 0;
            for i in squares[s]..=n {
                dp[s][i] = dp[s][i - squares[s]].saturating_add(1).min(dp[s + 1][i]);
            }
        }

        dp[0][n]
    }
    /// 23:41-23:43
    pub fn num_squares(n: i32) -> i32 {
        println!("num_squares({})", n);
        let n = n as usize;
        let max_root = (n as f64).sqrt() as usize;
        let squares = (1..=max_root).map(|i| i * i).collect::<Vec<_>>();
        let mut dp = vec![i32::MAX; n + 1];
        for s in 0..squares.len() {
            dp[0] = 0;
            for i in squares[s]..=n {
                dp[i] = dp[i].min(dp[i - squares[s]].saturating_add(1));
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn  n_1() { assert_eq!(Solution::num_squares( 1), 1); }
    #[rustfmt::skip] #[test] fn  n_2() { assert_eq!(Solution::num_squares( 2), 2); } // 1+1
    #[rustfmt::skip] #[test] fn  n_3() { assert_eq!(Solution::num_squares( 3), 3); } // 1+1+1
    #[rustfmt::skip] #[test] fn  n_4() { assert_eq!(Solution::num_squares( 4), 1); }
    #[rustfmt::skip] #[test] fn  n_5() { assert_eq!(Solution::num_squares( 5), 2); } // 4+1
    #[rustfmt::skip] #[test] fn  n_6() { assert_eq!(Solution::num_squares( 6), 3); } // 4+1+1
    #[rustfmt::skip] #[test] fn  n_7() { assert_eq!(Solution::num_squares( 7), 4); } // 4+1+1+1
    #[rustfmt::skip] #[test] fn  n_8() { assert_eq!(Solution::num_squares( 8), 2); } // 4+4
    #[rustfmt::skip] #[test] fn  n_9() { assert_eq!(Solution::num_squares( 9), 1); }
    #[rustfmt::skip] #[test] fn n_10() { assert_eq!(Solution::num_squares(10), 2); } // 9+1
    #[rustfmt::skip] #[test] fn n_11() { assert_eq!(Solution::num_squares(11), 3); } // 9+1+1
    #[rustfmt::skip] #[test] fn n_12() { assert_eq!(Solution::num_squares(12), 3); } // 4+4+4
    #[rustfmt::skip] #[test] fn n_13() { assert_eq!(Solution::num_squares(13), 2); } // 9+4
    #[rustfmt::skip] #[test] fn n_14() { assert_eq!(Solution::num_squares(14), 3); } // 9+4+1
    #[rustfmt::skip] #[test] fn n_15() { assert_eq!(Solution::num_squares(15), 4); } // 9+4+1+1
    #[rustfmt::skip] #[test] fn n_16() { assert_eq!(Solution::num_squares(16), 1); }

    /// If getting stack overflow: add RUST_MIN_STACK=134217728 (2 ** 27) to env:
    /// RUST_MIN_STACK=134217728 cargo test --lib d20_2
    #[rustfmt::skip] #[test] fn  n_9999() { assert_eq!(Solution::num_squares( 9999), 4); }
    #[rustfmt::skip] #[test] fn n_10000() { assert_eq!(Solution::num_squares(10000), 1); }
}
