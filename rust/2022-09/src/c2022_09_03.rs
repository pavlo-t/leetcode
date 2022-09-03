#![allow(dead_code)]
//! \#967. Numbers With Same Consecutive Differences
//! ================================================
//!
//! <https://leetcode.com/problems/numbers-with-same-consecutive-differences>
//!
//! Return all __non-negative__ integers of length `n` such that
//! the absolute difference between every two consecutive digits is `k`.
//!
//! Note that __every__ number in the answer __must not__ have leading zeros.
//! For example, `01` has one leading zero and is invalid.
//!
//! You may return the answer in __any order__.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```
//! # use c2022_09::c2022_09_03::*;
//! assert_eq!(Solution::nums_same_consec_diff(3, 7), [181, 292, 707, 818, 929]);
//! ```
//!
//! __Explanation:__ Note that `070` is not a valid number, because it has leading zeroes.
//!
//! ###### Example 2:
//!
//! ```
//! # use c2022_09::c2022_09_03::*;
//! assert_eq!(
//!     Solution::nums_same_consec_diff(2, 1),
//!     [10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
//! );
//! ```
//!
//! ##### Constraints
//!
//! - `2 <= n <= 9`
//! - `0 <= k <= 9`

pub struct Solution;
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        fn rec(first_digit: i32, n: i32, k: i32) -> Vec<i32> {
            match n {
                0 => vec![],
                1 => vec![first_digit],
                n => {
                    let mut result = vec![];
                    let num = first_digit * 10i32.pow(n as u32 - 1);
                    if first_digit - k >= 0 {
                        for next in rec(first_digit - k, n - 1, k) {
                            result.push(num + next);
                        }
                    }
                    if k != 0 && first_digit + k < 10 {
                        for next in rec(first_digit + k, n - 1, k) {
                            result.push(num + next);
                        }
                    }

                    result
                }
            }
        }

        let mut result = vec![];
        for first_digit in 1..10 {
            result.append(&mut rec(first_digit, n, k));
        }
        result
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn n_2_k_0() { assert_eq!(Solution::nums_same_consec_diff(2,0), [11, 22, 33, 44, 55, 66, 77, 88, 99]); }
    #[test] fn n_3_k_0() { assert_eq!(Solution::nums_same_consec_diff(3,0), [111,222,333,444,555,666,777,888,999]); }

    #[test] fn n_2_k_1() { assert_eq!(Solution::nums_same_consec_diff(2,1), [10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]); }

    #[test] fn n_2_k_4() { assert_eq!(Solution::nums_same_consec_diff(2,4), [15,     26, 37, 40, 48, 51, 59, 62, 73, 84,     95]); }
    #[test] fn n_3_k_4() { assert_eq!(Solution::nums_same_consec_diff(3,4), [151,159,262,373,404,484,515,595,626,737,840,848,951,959]); }

    #[test] fn n_2_k_5() { assert_eq!(Solution::nums_same_consec_diff(2,5), [16, 27, 38, 49, 50, 61, 72, 83, 94]); }
    #[test] fn n_3_k_5() { assert_eq!(Solution::nums_same_consec_diff(3,5), [161,272,383,494,505,616,727,838,949]); }

    #[test] fn n_2_k_7() { assert_eq!(Solution::nums_same_consec_diff(2,7),         [18, 29, 70, 81, 92]); }
    #[test] fn n_3_k_7() { assert_eq!(Solution::nums_same_consec_diff(3,7),         [181,292,707,818,929]); }

    #[test] fn n_2_k_8() { assert_eq!(Solution::nums_same_consec_diff(2,8),             [19, 80, 91]); }
    #[test] fn n_3_k_8() { assert_eq!(Solution::nums_same_consec_diff(3,8),             [191,808,919]); }

    #[test] fn n_2_k_9() { assert_eq!(Solution::nums_same_consec_diff(2,9),                 [90]); }
    #[test] fn n_3_k_9() { assert_eq!(Solution::nums_same_consec_diff(3,9),                 [909]); }
}
