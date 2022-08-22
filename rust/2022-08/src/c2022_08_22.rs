#![allow(dead_code)]
//! \#342. Power of Four
//! ====================
//!
//! Given an integer `n`, return _`true` if it is a power of four. Otherwise, return `false`_.
//!
//! An integer `n` is a power of four, if there exists an integer `x` such that `n == 4**x`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_22::*;
//! assert_eq!(Solution::is_power_of_four(16), true);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_22::*;
//! assert_eq!(Solution::is_power_of_four(5), false);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_22::*;
//! assert_eq!(Solution::is_power_of_four(1), true);
//! ```
//!
//! ##### Constraints
//!
//! - `-2**31 <= n <= 2**31 - 1`
//!
//! ##### Follow up
//!
//! Could you solve it without loops/recursion?

pub struct Solution;
impl Solution {
    /// Loop
    pub fn is_power_of_four_v1(mut n: i32) -> bool {
        if n < 1 {
            false
        } else {
            while n % 4 == 0 {
                n /= 4;
            }
            n == 1
        }
    }

    /// Using `log` and `pow`
    pub fn is_power_of_four_v2(n: i32) -> bool {
        n > 0 && n == 4i32.pow((n as f64).log(4.0) as u32)
    }

    /// Bit counting
    pub fn is_power_of_four(n: i32) -> bool {
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
    }
}
