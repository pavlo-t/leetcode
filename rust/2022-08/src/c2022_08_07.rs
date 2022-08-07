#![allow(dead_code)]
//! \#1220. Count Vowels Permutation
//! ================================
//!
//! <https://leetcode.com/problems/count-vowels-permutation>
//!
//! Given an integer `n`, your task is to count how many strings of length `n` can be formed under the following rules:
//!
//! - Each character is a lower case vowel (`'a'`, `'e'`, `'i'`, `'o'`, `'u'`)
//! - Each vowel `'a'` may only be followed by an `'e'`.
//! - Each vowel `'e'` may only be followed by an `'a'` or an `'i'`.
//! - Each vowel `'i'` __may not__ be followed by another `'i'`.
//! - Each vowel `'o'` may only be followed by an `'i'` or a `'u'`.
//! - Each vowel `'u'` may only be followed by an `'a'`.
//!
//! Since the answer may be too large, return it modulo `1_000_000_007`.
//!
//! ##### Constraints
//!
//! - `1 <= n <= 2 * 10_000`
//!
//! ##### Examples
//!
//! __Example 1__
//!
//! ```
//! # use c2022_08::c2022_08_07::Solution;
//! assert_eq!(Solution::count_vowel_permutation(1), 5);
//! ```
//!
//! __Explanation:__ All possible strings are: `"a"`, `"e"`, `"i"`, `"o"` and `"u"`.
//!
//! __Example 2__
//!
//! ```
//! # use c2022_08::c2022_08_07::Solution;
//! assert_eq!(Solution::count_vowel_permutation(2), 10);
//! ```
//!
//! __Explanation:__
//! All possible strings are: `"ae"`, `"ea"`, `"ei"`, `"ia"`, `"ie"`, `"io"`, `"iu"`, `"oi"`, `"ou"` and `"ua"`.
//!
//! __Example 3__
//!
//! ```
//! # use c2022_08::c2022_08_07::Solution;
//! assert_eq!(Solution::count_vowel_permutation(5), 68);
//! ```

pub struct Solution;
impl Solution {
    /// Recursion
    pub fn count_vowel_permutation_v1(n: i32) -> i32 {
        macro_rules! rec {
            ($n:expr,$($c:expr),+) => {{
                let mut result = 0;
                $(
                    result += rec($n, $c);
                    result %= 1_000_000_007;
                )+
                result
            }};
        }

        fn rec(n: i32, c: char) -> i32 {
            if n == 1 {
                1
            } else {
                let p = n - 1;
                match c {
                    'a' => rec!(p, 'e'),
                    'e' => rec!(p, 'a', 'i'),
                    'i' => rec!(p, 'a', 'e', 'o', 'u'),
                    'o' => rec!(p, 'i', 'u'),
                    'u' => rec!(p, 'a'),
                    _ => unreachable!(),
                }
            }
        }

        rec!(n, 'a', 'e', 'i', 'o', 'u')
    }

    /// Recursion with memo
    pub fn count_vowel_permutation_v2(n: i32) -> i32 {
        macro_rules! rec {
            ($n:expr,$m:expr,$($c:expr),+) => {{
                let mut result = 0;
                $(
                    result += rec($n, $m, $c);
                    result %= 1_000_000_007;
                )+
                result
            }};
        }

        /// a = 0, e = 1, i = 2, o = 3, u = 4
        fn rec(n: usize, memo: &mut Vec<Vec<i32>>, c: usize) -> i32 {
            if n == 0 {
                1
            } else {
                if memo[n][c] == -1 {
                    memo[n][c] = match c {
                        0 => rec!(n - 1, memo, 1),
                        1 => rec!(n - 1, memo, 0, 2),
                        2 => rec!(n - 1, memo, 0, 1, 3, 4),
                        3 => rec!(n - 1, memo, 2, 4),
                        4 => rec!(n - 1, memo, 0),
                        _ => unreachable!(),
                    }
                }
                memo[n][c]
            }
        }

        let n = n as usize - 1;
        let mut memo = vec![vec![-1; 5]; n + 1];

        rec!(n, &mut memo, 0, 1, 2, 3, 4)
    }

    /// DP
    pub fn count_vowel_permutation_v3(n: i32) -> i32 {
        macro_rules! add_mod {
            ($($c:expr),+) => {{
                let mut result = 0;
                $(
                    result += $c;
                    result %= 1_000_000_007;
                )+
                result
            }};
        }

        let n = n as usize;
        let mut dp = vec![vec![1; 5]; n];

        for n in 1..n {
            dp[n][0] = dp[n - 1][1];
            dp[n][1] = add_mod!(dp[n - 1][0], dp[n - 1][2]);
            dp[n][2] = add_mod!(dp[n - 1][0], dp[n - 1][1], dp[n - 1][3], dp[n - 1][4]);
            dp[n][3] = add_mod!(dp[n - 1][2], dp[n - 1][4]);
            dp[n][4] = dp[n - 1][0];
        }

        let last = &dp[n - 1];
        add_mod!(last[0], last[1], last[2], last[3], last[4])
    }

    /// DP constant memory
    pub fn count_vowel_permutation_v4(n: i32) -> i32 {
        macro_rules! add_mod {
            ($($c:expr),+) => {{
                let mut result = 0;
                $(
                    result += $c;
                    result %= 1_000_000_007;
                )+
                result
            }};
        }

        let mut prev = [1; 5];
        let mut curr = [1; 5];

        for _ in 1..n {
            curr[0] = prev[1];
            curr[1] = add_mod!(prev[0], prev[2]);
            curr[2] = add_mod!(prev[0], prev[1], prev[3], prev[4]);
            curr[3] = add_mod!(prev[2], prev[4]);
            curr[4] = prev[0];

            std::mem::swap(&mut curr, &mut prev);
        }

        add_mod!(prev[0], prev[1], prev[2], prev[3], prev[4])
    }

    /// DP constant memory
    pub fn count_vowel_permutation(n: i32) -> i32 {
        macro_rules! add_mod {
            ($($c:expr),+) => {{
                let mut result = 0;
                $(
                    result += $c;
                    result %= 1_000_000_007;
                )+
                result
            }};
        }

        let mut dp = [1; 5];

        for _ in 1..n {
            dp = [
                dp[1],
                add_mod!(dp[0], dp[2]),
                add_mod!(dp[0], dp[1], dp[3], dp[4]),
                add_mod!(dp[2], dp[4]),
                dp[0],
            ];
        }

        add_mod!(dp[0], dp[1], dp[2], dp[3], dp[4])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a,e,i,o,u
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::count_vowel_permutation(1), 5); }

    /// ae,
    /// ea,ei,
    /// ia,ie,io,iu,
    /// oi,ou,
    /// ua
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::count_vowel_permutation(2), 10); }

    /// aea,aei,
    /// eae, eia,eie,eio,eiu,
    /// iae, iea,iei, ioi,iou, iua,
    /// oia,oie,oio,oiu, oua,
    /// uae
    ///
    /// rec(_, 1) = 1
    ///
    /// rec(a, 2) = 1
    /// rec(e, 2) = 2
    /// rec(i, 2) = 4
    /// rec(o, 2) = 2
    /// rec(u, 2) = 1
    ///
    /// rec(a, 3) = rec(e, 2)
    /// rec(e, 3) = rec(a, 2) + rec(i, 2)
    /// rec(i, 3) = rec(a, 2) + rec(e, 2) + rec(o, 2) + rec(u, 2)
    /// rec(o, 3) = rec(i, 2) + rec(u, 2)
    /// rec(u, 2) = rec(a, 2)
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::count_vowel_permutation(3), 19); }

    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::count_vowel_permutation(5), 68); }

    /// Got stack overflow with default stack
    #[test]
    fn n_10000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                assert_eq!(Solution::count_vowel_permutation(10000), 76_428_576);
            })
            .unwrap();
        child.join().unwrap();
    }
}
