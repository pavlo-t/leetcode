#![allow(dead_code)]
//! \#869. Reordered Power of 2
//! ===========================
//!
//! <https://leetcode.com/problems/reordered-power-of-2>
//!
//! You are given an integer `n`.
//! We reorder the digits in any order (including the original order) such that the leading digit is not zero.
//!
//! Return _`true` if and only if we can do this so that the resulting number is a power of two_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_26::*;
//! assert_eq!(Solution::reordered_power_of2(1), true);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_26::*;
//! assert_eq!(Solution::reordered_power_of2(10), false);
//! ```
//!
//! ##### Constraints:
//!
//! - `1 <= n <= 1_000_000_000`

pub struct Solution;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digit_counts = |i: i32| {
            i.to_string()
                .bytes()
                .map(|b| (b - b'0') as usize)
                .fold([0; 10], |mut counts, i| {
                    counts[i] += 1;
                    counts
                })
        };

        let n_counts = digit_counts(n);

        (0..30)
            .map(|shift| 1 << shift)
            .map(digit_counts)
            .any(|power_of_2| power_of_2 == n_counts)
    }
}
