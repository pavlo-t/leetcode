#![allow(dead_code)]
//! \#326. Power of Three
//! =====================
//!
//! <https://leetcode.com/problems/power-of-three>
//!
//! Given an integer `n`, return _`true` if it is a power of three. Otherwise, return `false`_.
//!
//! An integer `n` is a power of three, if there exists an integer `x` such that `n == 3**x`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_24::*;
//! assert_eq!(Solution::is_power_of_three(27), true);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_24::*;
//! assert_eq!(Solution::is_power_of_three(0), false);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_24::*;
//! assert_eq!(Solution::is_power_of_three(9), true);
//! ```
//!
//! ##### Constraints
//!
//! - `-2**31 <= n <= 2**31 - 1`
//!
//! ##### Follow up
//!
//! Could you solve it without loops / recursion?

pub struct Solution;
impl Solution {
    /// Using `log(3.0)`
    pub fn is_power_of_three_v1(n: i32) -> bool {
        3i32.pow((n as f64).log(3.).round() as u32) == n
    }

    /// <https://leetcode.com/problems/power-of-three/solution/#approach-4-integer-limitations>
    pub fn is_power_of_three(n: i32) -> bool {
        const MAX_3_POW_I32: i32 = 3i32.pow(19);

        n > 0 && MAX_3_POW_I32 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_27() {
        assert_eq!(Solution::is_power_of_three(27), true);
    }

    #[test]
    fn n_0() {
        assert_eq!(Solution::is_power_of_three(0), false);
    }

    #[test]
    fn n_9() {
        assert_eq!(Solution::is_power_of_three(9), true);
    }

    #[test]
    fn n_243() {
        assert_eq!(Solution::is_power_of_three(243), true);
    }
}
